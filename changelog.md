Changelog
=
v0.3.7 Landing gear warning, adjustable stf deadband and some bug fixes
- Warning function: Landing gear forgotten to extend #27
- Add an adjustable audio deadband setting #87
- Calibration Sensor orientation, new procedure #91
- Sensor box settings #92
- The frontend uses gps_track instead of euler_yaw for the wind arrows #94

v0.3.6 Support for some Hardware Pins to Control the Front End
- Button input to switch between vario and speed-to-fly mode #84
- Input for automatic reduction of water ballast #67
- Output to control canopy flasher #81
- The centering aid is not distinguishing the circle direction correctly #85
- Sensorbox remote edit: do not transfer changed data immediately #79
- Sensorbox remote edit rad <-> deg #78

v0.3.5 Remote controllability via NMEA and optimized display
- Battery symbol switching rapidly #72
- Centering aid has points that are too small #73
- Tail in the single-arrow wind display is difficult to see #74
- Aircraft symbol in the wind display should be optional or removed #75
- Extensioin of the NMEA Interface #76
- Remote control of the fronend by XCSoar #77
- Change from probe-run to probe-rs for debuggung
- Separate output of RTIC utilization and trace messages
- Deactivate watchdog in debug mode
- Adjustments to Rust 2024 / rustc 1.85
- Create images with control elements (simulator)

v0.3.4 Support for Sensorbox Configuration, Polar Editor and more
- Support for Sensorbox Configuration #71, see also
  - [CAN Bus Specification](https://github.com/larus-breeze/doc_larus/blob/master/documentation/can_details/object_directory/sensorbox.md)
  - Flight Player [add log window for setting commands](https://github.com/larus-breeze/sw_tools/issues/20)
  - Flight Player [simulation of the sensor box settings](https://github.com/larus-breeze/sw_tools/issues/21)
- Editor for Glider Polars #68
- Add AS 33 Polar #69
- Bugfix: Values calculated by the Frontend Differs on Two-Seater Displays #64

v0.3.3 Extended Setting Options and Nicer Menues
- Introduction of user profiles #56
- Settings can be deleted per user profile #12
- Device can be reset to factory settings #12
- Flight menu now inside the Vario display (Larus V2 display) #60
- Volume and Mac Cready value can be changed directly #65
- Display selection (vario, horizon) moved to the flight menu #65

v0.3.2.5 Bug fix "AHRS display flickers massively"
- better performance to horizon to avoid flickering
- make fn draw_line_unchecked() unsafe to reflect missing range checks

v0.3.2 Add Thermal Assistants and Two-Arrow Wind Display 
- add thermal assistant "dotted"
- add thermal assistant "spider"
- add hawk-like two arrow wind display
- bugfix sound hang-up in debog mode (Larus V1)
- bugfix incrementing volume from 30 to 31 show anomalies (Larus V1 and Larus V2)
- bugfix hangup when updated fw version has fewer persistence data

v.0.3.1.3 Bug fix "Firmware Update via SdCard broken"
- This problem only affects the Larus Frontend V2 hardware.
- The error already existed in the firmware v.0.3.0.0
- The firmware update will only work as expected when upgrading from v.0.3.1.3 to a higher version. 

v.0.3.1 EMC Optimization, new can protocol and some bug fixes
- show firmware version at startup on info_1 line
- filter viewables depending on position
- fix crash in debug mode
- longer wait times when initialization the LCD
- EMC Optimization: setting LTDC gpio output buffers to low speed
- add new can protocol for sensorbox and gps device
- set vario needle to dark red

v.0.3.0 Support the Larus V2 hardware
- Larus V2 hardware with large display works
- Add menu infrastructure
- Add the option to customize displayed information
- Enables the display of flight level, UTC time, average climb rate, true course, drift angle and wind/avg wind
- The display can be rotated by 90°, 180°, and 270°
- The audio frequency can be customized
- Change the time constants for average climb rate and speed to fly

v.0.2.1 Artificial horizon
- Keyboard is now individual per display
- Editor as independent component
- Additional display: Artificial horizon

v.0.2.0 Enable NMEA interface
- $GPRMC, $GPGGA sentences (GPS)
- $PLARV, $PLARA, $PLARW, $PLARB, $PLARD sentences(Sensorbox)
- $PLARS sentences (Settings mac_cready, waterballast, ...)
- Reorganise the time handling and introduce 1 ms time slices
- Remove flickering during firmware update (Larus hardware)

v.0.1.5 Bug fix release
- fix color half/empty bat symbol
- add some can datagrams
- fix hang up of air avionics display

v.0.1.4 Optimisation of readability
- Introduction of bright mode
- Bold and bigger fonds
- Bat symbol for voltage representation
- Sat symbol for can bus and gps reception
- ST7789 driver for the new air avionics hw (not testet)

v0.1.3 Optimized usability - improved crash reports
- Write watchdog and panic events to sdcard inluding time and date
- Add Ventus 2 15m polar
- Change vario scale color to white
- The color of the wind arrow changes between flying straight ahead and thermaling
- Set wind direction and speed to zero when not flying
- Volume control range is now 50 dB instead of 30 dB
- Show firmware version on start up

v0.1.2 Detail optimisation
- Speed to fly is now low-pass filtered
- Air Avionics: no LCD flickering during firmware update

v0.1.1 First updateable release
- Vario display based on the lagacy can bus protocol
- Wind display and wind drift display
- Average climb rate with classic PT1 filter
- Display of the total climb rate of the last updraft
- Speed to fly display
- Speed to fly command display
- Vario tone and speed to fly tone on Larus frontend
- Remote control function for installation in a two-seater
- Power failure-proof storage of settings in EEPROM
- SW update function with SD cards for Larus and Air Avionics hardware

v0.1.0 Initial test release
