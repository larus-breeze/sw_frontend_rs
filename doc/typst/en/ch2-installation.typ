#import "../manual.typ": *

= #hr[Installation]
== #hr[Contents of the Package]

#tr[The following items are included:

- LARUS Vario Display
- Mounting screws
- 1:1 standard RJ45 cable
- Micro SD card with adapter
- D-Sub 9-pin solder-on connector and housing
- 1.5 mm hex key for fitting the rotary knobs]

== #hr[Quick Start Guide]

#tr[In some cases, the default settings can be used and the device can be
commissioned in a simplified manner. In such cases, it is sufficient to
observe the following points:

+ Please remove both knobs using a 1.5 mm hex key (included 
  in the scope of delivery).
+ Secure the LARUS Vario display with three screws in a 57 mm recess 
  in the instrument panel. It can also be mounted rotated by 90°, 180° and 270°.
+ Fit both rotary knobs
+ Connect the CAN port of the LARUS Vario display and the LARUS sensor unit using
  the 1:1 patch cable included in the delivery.
+ Switch on LARUS.
+ Check that the satellite icon on the screen is yellow or green and that the current 
  heading is displayed.
+ Select a suitable polar curve for your glider or create one.
+ Your LARUS Vario Display is now ready for flight.]    

== #hr[Design and Function]

#tr[The LARUS Vario Display shows the data measured and calculated by LARUS. LARUS is an
advanced variometer with a real-time wind measurement function. It features state-of-the-art pressure sensors,
an advanced IMU and GNSS receivers to capture precise flight data. The key
features of the display are:

- Round display for standard 57 mm instrument panel cut-outs
- Bright and colourful screen
- Lightweight, compact design with black anodised aluminium housing
- Two-stage rotary knob with push-button function for changing settings and
            accessing menus 1.5 mm hex key.

The LARUS Vario Display is developed and continuously improved by Prof. Dr Klaus Schaefer, Maximilian Betz, Winfried Simon,
Peter Simon and the SteFly team. You are welcome to 
contribute to its development by submitting suggestions for improvement or reporting any issues.]

#link("https://github.com/larus-breeze")[github.com/larus-breeze]

== #hr[System Configurations]<system-configurations>
=== #hr[Configuration in the Single-Seater]

#figure(
    image("/img/config-singleseater.jpg", height: 8cm),
    caption: [#hr[Wiring Diagram for Single-Seater Configuration]],
)<single-seater-configuration>

#tr[The navigation computer is connected to the LARUS Vario display (Option 1). This is often
the simplest option. Alternatively, however, the navigation computer can also be connected to the LARUS
sensor unit (Option 2).]

=== #hr[Two-Seater Configuration]

#figure(
    image("/img/config-doubleseater.jpg", height: 8cm),
    caption: [#hr[Wiring Diagram for Two-Seater Configuration]],
)<double-seater-configuration>

#tr[In a two-seater aircraft, the navigation computers are typically connected to the LARUS Vario displays.
However, it is also possible to connect the navigation computers to the LARUS sensor unit.
Data is exchanged between the LARUS components and distributed to the
navigation computers, ensuring that no information is lost.]

== #hr[Connectors and Cabling]
=== #hr[Connectors on the Back of the Device]

#figure(
    image("/img/connectors-overview.jpg", width: 6cm),
    caption: [#hr[View from the Back of the Device]],
)<connectors-overview>

#tr[The rear of the LARUS Vario display houses the connectors for CAN, RS232 and the
inputs/outputs. It also features the slot for inserting the SD card, as well as the 3.5 mm jack
for the audio output.]

=== #hr[CAN and RS232 Ports]

#figure(
    image("/img/connectors-rj45.svg", width: 6cm),
    caption: [#hr[RJ45 Connectors in Detail]],
)<connectors-rj45>

#h(5mm)
#align(left, block[
    #figure(
        table(
            columns: (auto, auto, auto),
            align: (center, left, left),
            table.hline(),
            table.header(
                [*Pin*], [*CAN*], [*RS232*],
            ),
            table.hline(),
            [1], [GND (internally connected)],    [GND (internally connected)],
            [2], [GND (internally connected)],    [GND (internally connected)],
            [3], [NC],                            [RS232-1-RX],
            [4], [CAN Low],                       [RS232-1-TX],
            [5], [CAN High],                      [NC],
            [6], [NC],                            [NC],
            [7], [VCC [9-28V DC] ],               [VCC [9-28V DC] ],
            [8], [VCC [9-28V DC] ],               [VCC [9-28V DC] ],
            table.hline(),
        ),
        caption: [#hr[Pin Configuration for CAN and RS232 RJ45]],
    )
])

=== GPIO / D-SUB 9

#tr[Several additional switches, sensors and devices can be connected via the D-Sub connector.
The following diagram shows the inside of the LARUS Vario Display’s connector.]

#figure(
    image("/img/connector-dsub9.png", width: 5cm),
    caption: [#hr[DSUB-9 Connector]],
)<connectors-dsub9>

#h(5mm)
#align(left, block[
    #figure(
        table(
            columns: (auto, auto, auto),
            align: (center, left, left),
            table.hline(),
            table.header(
                [*Pin*], [*Name*], [*I/O, Ground*],
            ),
            table.hline(),
            [1], [GND],                       [Ground],
            [2], [DI3 - Gear],                [Input],
            [3], [DI1 - Drain Ballast],       [Input],
            [4], [DO2],                       [Output],
            [5], [GND],                       [Ground],
            [6], [DI4 - Speed Breakes],       [Input],
            [7], [DI2 - Speed to Fly (StF)],  [Input],
            [8], [GND],                       [Ground],
            [9], [D01 - Canopy Flasher ],     [Output],
            table.hline(),
        ),
        caption: [#hr[GPIO / D-SUB 9 Pin Configuration]],
    )
])

#tr[To make it easier to identify the pins, these numbers are also stamped onto the socket (included
in the scope of delivery). Once the wiring is complete, the settings must be configured in the LARUS Vario display.

Meaning of the pin designations:
  - Ground: Connected to negative
  - Input: Input for a switch (open or closed)
  - Output: Output is opened/closed by the display

The Input and Output pin functions must be configured accordingly; see @settings.]

=== #hr[Audio] 

#tr[An audio socket is provided for connecting a speaker with a 3.5 mm jack plug.
The speaker’s impedance must be between 4 and 8 Ω (maximum output of 3
W at 4 Ω).]

#picnote("/img/pictograph-yellow-warning.svg")[
    #tr[A single speaker must not be connected to more than one device.]
]

=== #hr[SD Card]

#tr[The device has an SD card slot for firmware updates.]

#picnote("/img/pictograph-yellow-warning.svg")[
    #tr[As SD card adapters may damage the LARUS Vario Display, we 
    accept no liability for any damage caused by their use.]
]

=== #hr[Cabling]

#tr[When a serial connection (RS232) is established between the SteFly NAV and the LARUS Vario Display,
the MC value settings, for example, are synchronised/transferred between the devices.
In addition, the LARUS Vario Display processes inputs from a Speed-to-Fly/Vario switch,
which is connected directly to the SteFly remote control stick. The following steps are
required:

- Connect devices with a cable:
  - The CAN port of the LARUS sensor unit must be connected to the CAN port of the 
    LARUS Vario Display via a 1:1 patch cable (in two-seater configurations, using the cable box).
  - Devices running XCSoar can be connected via a crossed RX/TX patch cable between the 
    RS232 port of the Vario display and the ttySx port of the SteFly NAV.
- Optionally, connect switches for detecting the release of water ballast and the 
  landing gear warning to the corresponding DSUB-9-pin and any GND pin. For the 
  landing gear warning, you have the option of using two separate, directly connected switches for 
  the landing gear and the airbrakes (recommended) or a cable with both 
  switches connected in series to the DSUB-9 connector.
- The SteFly remote control stick can be connected in two different ways:
  - Depending on the configuration: using a separate cable to DSUB-9 pin 7
  - Using the USB cable (always available)
- Please adjust the settings in XCSoar (version 7.44 or higher) / OpenSoar (7.43 or 
  higher): If the Vario display is connected directly to the SteFly NAV, please select the 
  corresponding ttyS port, baud rate 38400, Larus driver, and enable synchronisation (option) with 
  the device.
- Please adjust the settings in the LARUS Vario Display for the connected optional 
  switches.
  - Landing gear warning
  - Water ballast
  - Configuration of the speed to fly switch (if not switched automatically):
    - Input pin: To use an external switch (on the control stick, flap lever, etc.), 
      set the Vario control to “Input pin” and select the correct setting       
      in the StF pin configuration. (active: when open/closed)
    - NMEA for SteFly remote control stick. In addition to the settings in the Vario, you must 
      install an event definition file in XCSoar/OpenSoar (Configuration / System / 
      View / Language, Input / Events – enable Expert, click on Events – 
      Download – GLB-XCI-xcremote-XCNAV.xci). Exit the XCSoar configuration and 
      restart XCSoar.]

== #hr[CAN Termination]

#tr[The LARUS Vario Display and the LARUS sensor unit are connected to each other via the CAN bus.
CAN bus networks require terminating resistors at each end of the network. Therefore, all
devices have a built-in switch to activate the resistor:]

#figure(
    image("/img/can-termination.jpg", width: 16cm),
    caption: [#hr[Switches on the Devices for Terminating the CAN Bus]],
)<can-termination>

#h(5mm)
#align(left, block[
    #figure(
        table(
            columns: (auto, auto, auto, auto, auto),
            align: (left, center, center, center, center),
            table.hline(),
            table.header(
                [#hr[*Description*]], [#hr[*Front Display*]], [#hr[*Sensor Unit*]], [#hr[*CAN Splitter*]], [#hr[*Rear Display*]],
            ),
            table.hline(),
            [#hr[Single-Seater]],                                           [on],  [on],  [-],   [-],
            [#hr[Two-Seater, Larus Sensor Unit At the Front]],             [off], [on],  [off], [on],
            [#hr[Two-Seater, Larus Sensor Unit At the Rear]],            [on],  [on],  [off], [off],
            table.hline(),
        ),
        caption: [#hr[CAN Wiring Harness for Single-Seater/Two-Seater]],
    )
])

#tr[Please note: All LARUS sensor units delivered before March 2025 do not have
a CAN termination switch. The CAN termination resistors are always enabled by default.]

== #hr[External Backup]

#picnote("/img/pictograph-yellow-warning.svg")[
    #tr[The LARUS Vario Display is usually powered via a
    patch cable connected between the CAN ports on the LARUS. The LARUS must be protected by an
    external fuse (500 mA to max. 3 A), as is standard practice for all electrical equipment
    in aviation. If LARUS draws its power from another primary instrument
    (e.g. SteFly NAV via D-Sub connector), please ensure that the primary instrument is
    protected by an external fuse accordingly.]
]

== #hr[Installation Site]

#tr[The following image shows a typical installation of the LARUS Vario display in
the instrument panel of a glider.]

#figure(
    image("/img/panel.jpg", width: 10cm),
    caption: [#hr[Installation in the Instrument Panel]],
)<panel>

#tr[The display fits into a standard 57 mm cut-out and is secured with three M3 screws.]

#figure(
    image("/img/knobs-and-screws.jpg", width: 8cm),
    caption: [#hr[Fasteners]],
)<knobs-and-screws>

#tr[To install it, you need to remove the two knobs using a
1.5 mm hex key.]

== #hr[Installation Orientation]

#picnote("/img/pictograph-blue-cloud.svg")[
    #tr[If you intend to replace an existing device with the LARUS
    Vario Display, please check the desired mounting position before drilling the
    7.3 mm hole for the encoder, as the display housing is slightly asymmetrical.]
]

#tr[The display can be mounted at angles of 0°, 90°, 180° or 270°. Once the display has been mounted,
its orientation may need to be adjusted in the ‘Display Rotation’ menu.]

== #hr[Initial Commissioning and Functional Test]

#tr[To set up the system, please follow these steps:

+ Please check that the LARUS Vario Display is connected in accordance with the diagrams in  
  @system-configurations. 
+ Switch on the LARUS.
+ Please check that the LARUS Vario Display starts up and that a yellow or green 
  satellite icon is displayed. The Vario indicators should show slight movements around the
  zero position.
+ You should now configure the device. All configuration options are documented in detail in the 
  settings chapter.]

== #hr[Maintenance] 

#tr[The entire system contains no parts that require maintenance. To make a claim under the warranty, 
please contact SteFly directly.]

#picnote("/img/pictograph-yellow-warning.svg")[
    #tr[Opening the casing of the LARUS Vario display will invalidate the warranty.]
]

== #hr[Firmware Update]

#tr[The LARUS team is constantly improving the software and releasing firmware updates. To
update the firmware, follow these steps:

+ Switch off the device.
+ Save the new \*.bin file to the SD card supplied with the device and insert 
  it into the SD card slot on the back of the LARUS Vario Display.
+ Switch on the device.
+ When firmware is detected on the SD card, the display will remain black for approximately 3–5 seconds 
  before the message "#keep[Installing... Do NOT power off device]" appears.
+ The device will restart automatically. During the first 10 seconds, the firmware version will be displayed in the Info1 section
  .

The LARUS Vario Display will only install compatible firmware versions. If multiple firmware 
versions are stored on the card, the latest one will be installed.

If the installation fails, please repeat the process. If the
installation fails again, please use a different SD card. The SD card must
be at least 4 GB in size (SDHC type) and formatted with FAT32. The format must be compatible with
DOS/Windows 95 (not GPT).]
