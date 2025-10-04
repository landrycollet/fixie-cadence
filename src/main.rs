//extern crate ini;
use ini::Ini;
use regex::Regex;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::{Write, BufWriter};

pub struct Transmission {
    ratio: f64, // Chainring teeth count / Cog teeth count.
    wheel_diameter_m: f64 // Rear wheel diameter (including tyre) in meter.
}

impl Transmission {
    const SPEED_MS_TO_WHEEL_RPM_FACTOR: f64 = 60.0 / (std::f64::consts::PI); // 60 sec in a min.

    fn get_cadence_m_s(&self, speed_ms: f64) -> f64 {
        speed_ms * Self::SPEED_MS_TO_WHEEL_RPM_FACTOR / (self.wheel_diameter_m * self.ratio)
    }
}

fn main() {
    // Start by loading the transmission parameters from the conf.ini file.
    let conf = Ini::load_from_file("conf.ini").expect("No conf.ini file found!");
    let transmission_config = conf.section(Some("Transmission")).unwrap();
    let transmission_teeth_cog: f64 = transmission_config.get("nb_teeth_cog").unwrap().parse().unwrap();
    let transmission_teeth_chainring: f64 = transmission_config.get("nb_teeth_chainring").unwrap().parse().unwrap();
    let transmission_wheel_diam_m: f64 = transmission_config.get("rear_wheel_diameter_m").unwrap().parse().unwrap();
    println!("Loaded configration:");
    println!("Nb teeth cog: {}", transmission_teeth_cog);
    println!("Nb teeth chainring: {}", transmission_teeth_chainring);
    println!("Wheel size: {}", transmission_wheel_diam_m);
    let trans = Transmission {
        ratio: transmission_teeth_chainring / transmission_teeth_cog,
        wheel_diameter_m: transmission_wheel_diam_m
    };

    // Load input file.
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
      panic!("Usage: cargo run <input_file>");
    }
    let source_file_path = &args[1];
    if !is_tcx_file(source_file_path) { panic!("Not a valid file"); }

    // Create path for the output file: take the input file, remove the .tcx, append suffix with
    // .tcx extension.
    let mut output_file_path = String::from(&source_file_path[..(source_file_path.len() - 4)]);
    output_file_path.push_str("_with_cadence.tcx");

    let source_file = File::open(source_file_path).expect("Should be able to open {source_file_path}");
    let source_file = BufReader::new(source_file);

    let output_file = File::create(output_file_path).expect("Should be able to create file");
    let mut output_file = BufWriter::new(output_file);

    println!("\nStarting file parsing and generation...");
    let mut trackpoint_cadence: Option<i32> = None;
    let mut cadence_already_set_in_trackpoint = false;
    // The regex that will be used to extract the cadence string.
    // The regex needs to be compiled outside the loop for efficiency.
    let float_regex = Regex::new(r"([0-9]*[.])[0-9]+").unwrap();
    for line in source_file.lines() {
        let line = line.expect("Should be able to read line");

        if line.contains(":Speed")
        {
            // println!("Found SPEED data: Line: {line}");
            let speed_entry = String::from(&line);
            // Use the regex to extract the speed value.
            let speed_ms_str = float_regex.find(&speed_entry).unwrap().as_str();
            let speed_ms: f64 = speed_ms_str.parse().unwrap();
            // Compute the cadence from the value.
            trackpoint_cadence = Some(trans.get_cadence_m_s(speed_ms).trunc() as i32);
            println!("found speed {} m.s-1, calculated cadence is {} rpm",
                    speed_ms,
                    trackpoint_cadence.unwrap()
                    );
        }
        if line.contains("Cadence")
        {
            cadence_already_set_in_trackpoint = true;
        }
        if line.contains("</Trackpoint>")
        {
            // Only write cadence data point if it's not in the Trackpoint already.
            if !cadence_already_set_in_trackpoint {
                // Insert a line with the computed cadence, if it has been found previously.
                match trackpoint_cadence {
                    None => (),
                    Some(cadence) =>
                        // 12 spaces for correct indentation in Garmin files.
                        writeln!(output_file,
                                 "            <Cadence>{}</Cadence>",
                                 cadence).expect("Should be able to write cadence"),
                };
            }
            // Invalidate cadence point in memory.
            trackpoint_cadence = None;
            cadence_already_set_in_trackpoint = false;
        }
        writeln!(output_file, "{}", line).expect("Should be able to write to file");
    }
    println!("Complete!");
}


/// Ensure the file path points to a tcx file, checking the file extension.
fn is_tcx_file(path: &str) -> bool {
    let len = path.len();
    if len < 5 {
        return false
    }
    return &path[(len - 4)..] == ".tcx";
}

// -----------------------------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_cadence_700c_ratio_42_18() {
        let trans = Transmission {
            ratio: 42.0 / 18.0,
            wheel_diameter_m: 0.7
        };
        let speed_ms = 6.95; // Same as 25 km/h.
        let expected_trunc_result = 81.0; // 81.20150157749762.
        let result = trans.get_cadence_m_s(speed_ms).trunc();
        assert_eq!(result, expected_trunc_result);
    }

    #[test]
    fn test_is_tcx_file_real_file() {
        assert_eq!(is_tcx_file("/file.tcx"), true);
    }

    #[test]
    fn test_is_tcx_file_wrong_file() {
        assert_eq!(is_tcx_file("/file.txc"), false);
    }

    #[test]
    fn test_is_tcx_file_too_short_file() {
        assert_eq!(is_tcx_file("a.c"), false);
    }
}
