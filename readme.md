Firmware for Displaying the Larus Sensor Values
===============================================

General information
-------------------

This is a vario front end for gliders, which displays the measured values of the Larus sensor box. The display is modeled after a classic winter vario. The display is clear and easy to understand. It shows the values measured by the Larus sensor box.

The Larus Sensorbox uses a new method which calculates the actual climb rate from many sensor data, in particular GPS, pressure, acceleration, rotation and magnetic sensors are evaluated. As a result, the pilot receives a display that is instantaneous and authentic. For the first time, the vario display matches the pilot's sense of acceleration exactly. Another outstanding feature of the sensor box is the precise calculation of the current wind accurate to the second, which should be adequately displayed.

The following target systems are currently supported. Installation instructions for the development environment can be found there:

[A PC simulation](https://github.com/larus-breeze/sw_frontend_rs/tree/master/device/pc) (Linux, Windows) for development and testing
![tools](https://github.com/larus-breeze/sw_frontend_rs/assets/3678273/74c01117-cf99-40b7-b68e-ff5c3c36fc2b)

[Ad57 from Air Avionics](https://github.com/larus-breeze/sw_frontend_rs/tree/master/device/air_avionics_ad57)

<img src="https://github.com/user-attachments/assets/36f65970-831a-480e-8eea-c4b77a390265" width="300"><br /><br />

[Larus 57mm V1 Frontend](https://github.com/larus-breeze/sw_frontend_rs/tree/master/device/larus_frontend_v1) Cost-effective display with an STM32H7 processor, a reflective display and a housing that can be manufactured using standard 3D printers

<img src="https://github.com/user-attachments/assets/49be542f-0b2f-41f9-a855-876065db93e9" width="300"><br /><br />

[Larus 57mm V2 Frontend](https://github.com/larus-breeze/sw_frontend_rs/tree/master/device/larus_frontend_v2) Newly developed display with an STM32H7 processor and a bright 57 mm round display

<img src="https://github.com/user-attachments/assets/28192747-6cb0-42bd-bf88-59092df5014e" width="300"><br /><br />


The software is written in the Rust programming language. Special emphasis was placed on making the application portable and maintainable.
