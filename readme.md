Firmware for displaying the Larus sensor values
===============================================

General information
-------------------

This is a vario front end for gliders, which displays the measured values of the Larus sensor box. The display is modeled after a classic winter vario. The Larus Sensorbox uses a new method which calculates the actual climb rate from many sensor data, in particular GPS, pressure, acceleration, rotation and magnetic sensors are evaluated. As a result, the pilot receives a display that is instantaneous and authentic. For the first time, the vario display matches the pilot's sense of acceleration exactly. Another outstanding feature of the sensor box is the precise calculation of the current wind accurate to the second, which should be adequately displayed.

The following target systems are currently supported:
- A PC simulation (Linux, Windows) for development and testing
- Ad57 from Air Avionik (commercial 57mm built-in instrument)

![tools](https://github.com/larus-breeze/sw_frontend_rs/assets/3678273/74c01117-cf99-40b7-b68e-ff5c3c36fc2b)
The figure shows the simulation environment of the solution.

The software is written in the Rust programming language. Special emphasis was placed on making the application portable and maintainable. The applications for the target systems are located in the device directory.

The software is open source and can be used free of charge. See also the license conditions.
