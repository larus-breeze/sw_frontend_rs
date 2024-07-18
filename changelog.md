Changelog
=

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
