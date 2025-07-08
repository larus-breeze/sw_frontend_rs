Firmware for Displaying the Larus Sensor Values
===============================================

General information
-------------------

This is a vario front end for gliders that displays the measured values of the Larus sensor box. The display should be clear and easy to understand. It displays the values measured by the Larus Sensorbox.

The Larus Sensorbox uses a new method that calculates the actual climb rate from a large amount of sensor data, whereby GPS, pressure, acceleration, rotation and magnetic sensors in particular are evaluated. As a result, the pilot receives a prompt and authentic display. For the first time, the vario display matches the pilot's sense of acceleration. Another outstanding feature of the sensor box is the calculation of the current wind, which should be adequately displayed.

The following systems and hardware variants are currently supported. Installation instructions for the development environment can be found in the respective directory:

[Larus 57mm V2 Frontend](https://github.com/larus-breeze/sw_frontend_rs/tree/master/device/larus_frontend_v2) Newly developed display with an STM32H7 processor and a bright 57 mm round display

<img src="https://github.com/user-attachments/assets/28192747-6cb0-42bd-bf88-59092df5014e" width="300"><br /><br />

[Larus 57mm V1 Frontend](https://github.com/larus-breeze/sw_frontend_rs/tree/master/device/larus_frontend_v1) Cost-effective display with an STM32H7 processor, a reflective display and a housing that can be manufactured using standard 3D printers

<img src="https://github.com/user-attachments/assets/49be542f-0b2f-41f9-a855-876065db93e9" width="300"><br /><br />

[A PC simulation](https://github.com/larus-breeze/sw_frontend_rs/tree/master/device/sim) (Linux, Windows) for development and testing
![readme](https://github.com/user-attachments/assets/70e1fc85-923c-4050-878e-e151989f4b38)

The software is written in the Rust programming language.

