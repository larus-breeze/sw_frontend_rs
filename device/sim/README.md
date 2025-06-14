Larus Frontend Simulator
=
![Larus Frontend Simulator](https://github.com/larus-breeze/sw_frontend_rs/blob/master/device/sim/doc/screenshot_larus_frontend_simulator.png?raw=true)
Development, Testing and Documentation
-

The simulator uses the core libraries of the frontend firmware. This ensures that the display is rendered with pixel precision and all functions can be traced. Together with Flight Player and XCSoar, it is possible to accurately recreate scenarios from real flights in order to analyze the behavior of the firmware.
- During development, this helps to test new functions and see how they look and work.
- During testing, all functions of the firmware can be simulated and reproduced.
- Screenshots can be saved to supplement the documentation.

Installation
-
The installation of the Rust programming language is described in detail on the Rust homepage. There are no external dependencies on any components, so a simple ‘cargo run’ completely builds and executes the software. The software was developed and tested under Linux. It should also run under Windows and MAC OS.

In order to hear the Vario tone, the file ~/.asoundrc must be created under Linux so that the software knows which channel to output the sound on. The contents of this file depend on the PC on which the software is running and look something like this:

```
defaults.pcm.!card Generic
defaults.ctl.!card Generic
defaults.pcm.!device 7
defaults.ctl.!device 7
```
Details can be found on the Internet using the keywords ‘alsa’ and ‘asoundrc’.

Features
-

The simulation provides the following functions:
- Display of all screens with all details
- Operation of the device, including use of all menus and settings
- Generation of the audio signal
- Emulation of an EEPROM for storing frontend configuration values
- Simulation of NMEA communication (XCSoar, tcp, 127.0.0.1:4353)
- Simulation of CAN communication (Flight Player, udp, 127.0.0.1:5005) 
- The contents of all communication channels can be displayed, filtered, and saved
- Screenshots of the pure Vario display can be saved.

<img src="https://github.com/larus-breeze/sw_frontend_rs/blob/master/device/sim/doc/screenshot_vario.png?raw=true" alt="Vario" width="200" height="200">
 
All functions can be operated from both the keyboard and the user interface.

Keyboard Layout
-

```
⇒ Cursor right: Small Encoder right
⇐ Cursor left: Small Encoder left
⇑ Cursor up Big: Encoder right
⇓ Cursor down Big: Encoder left

F1 Log Window Run
F2 Log Window Pause
F3 Log Window Save
F4 Log window Clear

F5 Flight Menu
F6 Settings Menu
F7 Save Vario Screenshot
F8 Toggle Log Filter 'Idle Events'


F9 Toggle Log Filter 'NMEA In'
F10 Toggle Log Filter 'NMEA Out'
F11 Toggle Log Filter 'CAN In'
F12 Toggle Log Filter 'CAN Out'

<Enter> Enter selected Menu  Item
<ESC> Return from Menu

c Copy Vario Screenshot to Clipboard
q Quit Applicatiion
s Save Vario Screenshot to varioxx.png

1 Togle Input Pin 1
2 Togle Input Pin 2
3 Togle Input Pin 3
4 Togle Input Pin 4
```
Implementation
-

Here are a few notes on the implementation of this simulator
- The [slint](https://slint.dev/) package was used to create the user interface. This pure Rust library from a small company in Berlin makes it possible to develop software for Linux, Windows, Mac OS, Android, and even embedded systems using identical sources.
- The [arboard](https://github.com/1Password/arboard) package enables easy, platform-independent access to the clipboard.
- The audio sound was developed with the help of [tinyaudio](https://docs.rs/tinyaudio/latest/tinyaudio/). This minimal platform-independent interface is ideal for implementing this audio output.
- Without the above and many other software packages, it would not have been possible for me to develop this simulator.



