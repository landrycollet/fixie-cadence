# Fixie cadence

# Overview
This program allows you to modify a Garmin `tcx` file to add the pedaling cadence based on your speed, fixed transmission ratio and wheel size.

The Garmin head units record the pedaling cadence from a dedicated cadence sensor. Most of the time you do need a separate sensor because the pedals are not hardly linked to the rest of the transmission: your bike usually keeps rolling if you stop pedaling.

But since track and fixed-gear bikes don't have a free hub, the cadence can be extrapolated from the speed, a data that is generated either by the GPS of your Garmin head unit or an external speed sensor.

You just need to know your gear ratio (number of teeth on your chainring and cog), and the wheel diameter (including the tyre).

Then the program will provide you with a new tcx file that you can import to Garmin Connect or Strava.

# Disclaimer
This project is not affiliated with Garmin / Strava in any way.

Use with care as I can't guarantee your Garmin or Strava account won't be suspended for uploading tempered data files... Although I don't personally see how adding a virtual cadence, even if the numbers are crazy, would allow you to cheat at challenges.

# Usage
Open `conf.ini` and edit the values for `nb_teeth_cog`, `nb_teeth_chainring` and `wheel_diameter_m` to match your setup. For decimals values, use a `.`, not a `,`. Wheel diameter is to be set in meters, so for instance a 700C wheel with a 23mm tyre is around `0.685`.

Save.

Then, run the program with the following command:

```
cargo run -- <path to the tcx file>
```

It will generate the new tcx file along the input file, with a `_with_cadence` suffix.

# How it works
Let's first have a look at a typical Trackpoint structure with the fields that are of interest:

```
  <Trackpoint>
    ..
    <Extensions>
      <ns3:TPX>
        <ns3:Speed>7.334000110626221</ns3:Speed>
      </ns3:TPX>
    </Extensions>
  </Trackpoint>
```

The code parses the `.tcx` file and copy each lines into a new file. It also looks for a field with the tags `<ns3:Speed>` (well actually just `:Speed>` to filter other tags like `<MaximumSpeed>`).

It then uses a Regex to extract the value between the `<ns3:Speed>` tags. Speed is recorded in m.s<sup>-1</sup>.

Then this value is used to calculate the cadence for that trackpoint.

When the parser encounters a `</Trackpoint>` tag, if a cadence value has been computed for that trackpoint, it inserts it between `<Cadence>` tags just before closing the trackpoint entry.

# License and accountability
This code is licensed under MIT.

Copyright (c) 2025 Landry COLLET

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

# Bug report / Feedback
Feedback, as long as it is constructive, is welcome. Bear in mind that I made that project as an experimentation to get familiar with Rust, as I started learning the language, but I'm eager to get better at it!

I have tested this project successfully with the devices I have at hand, but you might experience some issues with your own setup. Please let me know if this happens, and provide as much informations as possible so that I can fix it.

# Developers section

## Testing
Run tests with
```
cargo test
```

## TODO
- [ ] Improve error handling and recovery.
- [ ] Document functions.
- [ ] Clean up / Set in different files.

## Follow up ideas
- [ ] Set up a web page that runs that script so that any user can upload his own file without dealing with the software.
- [ ] Find a way to have that algorithm as a virtual cadence sensor integrated to the Garmin head unit directly.
