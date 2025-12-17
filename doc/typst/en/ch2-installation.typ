#import "../manual.typ": *

= #hr[Installation]
== #hr[Scope of Delivery]

#tr[The following parts are included:

- LARUS Vario display
- Mounting screws
- 1:1 standard RJ45 cable
- Micro SD card with adapter
- D-Sub 9 solder plug and housing
- 1.5 mm hex key for mounting the rotary knobs]

== #hr[Quick Start Guide]

#tr[In some cases, the default settings can be used and the device can be put into operation in a
simplified manner. In this case, it is sufficient to observe the following
points:

+ Please remove both rotary knobs using a 1.5 mm hex key (included in the scope of delivery 
).
+ Secure the LARUS Vario display with three screws in a 57 mm recess 
  in the instrument panel. It can also be mounted at 90°, 180° and 270° angles.
+ Fit both rotary knobs
+ Connect the CAN port of the LARUS Vario display and the LARUS sensor unit using the
  1:1 patch cable supplied.
+ Switch on LARUS.
+ Check that the satellite symbol on the screen is yellow or green and that the current 
  course direction is displayed.
+ Select a suitable polar for your glider or create one.
+ Your LARUS Vario Display is now ready to fly.]	

== #hr[Design and Function]

#tr[The LARUS Vario Display shows the data measured and calculated by LARUS. LARUS is an
advanced variometer with real-time wind measurement capability. It features state-of-the-art pressure sensors,
an advanced IMU and GNSS receivers to capture accurate flight data. The key
features of the display are:

- Round display for 57 mm standard instrument panel cut-outs
- Bright and colourful screen
- Lightweight, compact design with black anodised aluminium housing
- Two-level rotary knob with push-button function for changing settings and
            accessing menus 1.5 mm hex key.

The LARUS Vario Display is developed and continuously improved by Prof. Dr. Klaus Schaefer, Maximilian Betz, Winfried Simon,
Peter Simon and the SteFly team. You are welcome to 
participate in the development and contribute suggestions for improvement or report problems.]

#link("https://github.com/larus-breeze")[github.com/larus-breeze]

== #hr[System Configurations]<system-configurations>
=== #hr[Configuration in the Single-Seater]

#figure(
    image("/img/config-singleseater.jpg", height: 8cm),
    caption: [#hr[Connection Diagram for Single-Seater Configuration]],
)<single-seater-configuration>

#tr[The navigation computer is connected to the LARUS Vario display (Option 1). This is often
the simplest option. Alternatively, the navigation computer can also be connected to the LARUS
sensor unit (Option 2).]

=== #hr[Configuration in the Two-Seater]

#figure(
    image("/img/config-doubleseater.jpg", height: 8cm),
    caption: [#hr[Connection Diagram for Two-Seater Configuration]],
)<double-seater-configuration>

#tr[In two-seaters, the navigation computers are typically connected to the LARUS Vario displays.
However, it is also possible to connect the navigation computers to the LARUS sensor unit.
The data is exchanged between the LARUS components and distributed to the navigation computers so that no information is lost.
The LARUS sensor unit is connected to the LARUS Vario displays and the LARUS navigation computers.]

== #hr[Connectors and Cabling]
=== #hr[Connector on the Rear of the Device]

#figure(
    image("/img/connectors-overview.jpg", width: 6cm),
    caption: [#hr[View from the Rear of the Device]],
)<connectors-overview>

#tr[The connections for CAN, RS232 and the inputs/outputs are located on the rear of the LARUS Vario display.
The slot for inserting the SD card and the 3.5 mm audio output jack are also located there.
The LARUS Vario display is equipped with a 4.3-inch colour touchscreen.]

=== #hr[CAN and RS232 Connections]

#figure(
    image("/img/connectors-rj45.svg", width: 6cm),
    caption: [#hr[The RJ45 Connectors in Detail]],
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
            [1], [GND (internally connected)],	[GND (internally connected)],
            [2], [GND (internally connected)],	[GND (internally connected)],
            [3], [NC], 						  	[RS232-1-RX],
            [4], [CAN Low], 					[RS232-1-TX],
            [5], [CAN High], 					[NC],
            [6], [NC], 							[NC],
            [7], [VCC [9-28V DC] ], 			[VCC [9-28V DC] ],
            [8], [VCC [9-28V DC] ], 			[VCC [9-28V DC] ],
            table.hline(),
        ),
        caption: [#hr[CAN and RS232 RJ45 Pin Assignment]],
    )
])

=== GPIO / D-SUB 9

#tr[Several additional switches, sensors and devices can be connected via the D-Sub connector.
The following diagram shows the view into the connector of the LARUS Vario display.]

#figure(
    image("/img/connector-dsub9.png", width: 6cm),
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
            [1], [GND],						[Ground],
            [2], [DI3 - Gear],				[Input],
            [3], [DI1 - Water Ballast], 	[Input],
            [4], [DO2], 					[Output],
            [5], [GND], 					[Ground],
            [6], [DI4 - Speed Breakes], 	[Input],
            [7], [DI2 - Speed to Fly], 		[Input],
            [8], [GND], 					[Ground],
            [9], [D01 - Canopy Flasher ], 	[Output],
            table.hline(),
        ),
        caption: [#hr[Pin Assignment GPIO / D-SUB 9]],
    )
])

#tr[To make it easier to identify the pins, these numbers are also stamped into the socket (included in the scope of delivery).
 After wiring, the settings must be made in the LARUS Vario display
.]

=== #hr[Audio] 

#tr[An audio jack is available for connecting a speaker with a 3.5 mm jack plug.
The internal resistance of the speaker must be between 4 and 8 Ω (max. output of 3
W at 4 Ω).]

#picnote("/img/pictograph-yellow-warning.svg")[
    #tr[A single loudspeaker must not be connected to more than one device.]
]

=== #hr[SD Card]

#tr[The device has an SD card slot for firmware updates.]

#picnote("/img/pictograph-yellow-warning.svg")[
    #tr[As SD card extensions can damage the LARUS Vario display, we accept 
    no liability for damage caused by their use.]
]

=== #hr[Cabling]

#tr[When a serial connection (RS232) is established between SteFly NAV and LARUS Vario Display,
the MC value settings, for example, are synchronised/transferred between the devices.
In addition, the LARUS Vario Display processes inputs from a speed-to-fly/vario switch,
which is directly connected to the SteFly remote control lever. The following steps are
required:

- Connect devices with cables:
  - The CAN port of the LARUS sensor unit must be connected to the CAN port of the 
    LARUS VARIO display via a 1:1 patch cable (for two-seater configurations using the cable box).
  - Devices running XCSoar can be connected via a crossed RX/TX patch cable between the 
    RS232 port of the vario display and the ttySx port of the SteFly NAV.
- Optionally, connect switches for detecting the release of water ballast and the 
  landing gear warning to the corresponding DSUB-9 pin and any GND pin. For the 
  landing gear warning, you have the option of using two separate, directly connected switches for 
  the landing gear and the brake flaps (recommended) or a cable with the two 
  switches in series to the DSUB-9 port.
- The SteFly remote control stick can be connected in two different ways:
  - Depending on the equipment: with a separate cable to DSUB-9 pin 7
  - Use of the USB cable (always available)
- Please adjust the settings in XCSoar (version 7.44 or higher) / OpenSoar (7.43 or 
  higher): If the vario display is connected directly to the SteFly NAV, please select the 
  corresponding ttyS port, baud rate 38400, Larus driver, synchronisation (option) with 
  device activated.
- Please adjust the settings in the LARUS Vario Display for the connected optional 
  switches.
  - Warning regarding the landing gear
  - Water ballast
  - Configuration of the speed to fly switch (if not switched automatically):
    - Input pin: To use an external switch (on the control stick, flap lever, etc.), 
      set the Vario control to "Input pin" and select the correct setting       
      in the StF pin configuration. (active: when open/closed)
- NMEA for SteFly remote control stick. In addition to the settings in the Vario, you must 
      install an event definition file in XCSoar/OpenSoar (Configuration / System / 
      View / Language, Input / Events – activate Expert, click on Events – 
      Download – GLB-XCI-xcremote-XCNAV.xci). Exit the XCSoar configuration and 
      restart XCSoar.]

== #hr[CAN Termination]

#tr[The LARUS Vario display and the LARUS sensor unit are connected to each other via the CAN bus.
CAN bus networks require terminating resistors at each end of the network. Therefore, all
devices have an integrated switch for activating the resistor:]

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
            [#hr[Two-Seater, Larus Front Sensor Unit]],             [off], [on],  [off], [on],
            [#hr[Two-Seater, Larus Rear Sensor Unit]],            [on],  [on],  [off], [off],
            table.hline(),
        ),
        caption: [#hr[CAN Termination Single/Double Seater]],
    )
])

#tr[Please note: All LARUS sensor units delivered before March 2025 do not have a CAN termination switch. The CAN termination resistors are always activated by default.
Please note: All LARUS sensor units delivered before March 2025 do not have a CAN termination switch. The CAN termination resistors are always activated by default.]

== #hr[External Backup]

#picnote("/img/pictograph-yellow-warning.svg")[
    #tr[As a rule, the LARUS Vario Display is powered via a
    patch cable between the CAN connections of LARUS. LARUS must be protected by an
    external fuse (500 mA to max. 3 A), as is standard practice for all electrical devices
    in aviation. If LARUS draws its power from another main instrument
    (e.g. SteFly NAV via D-Sub connector), please ensure that the main instrument is protected by
    an external fuse accordingly.]
]

== #hr[Installation Site]

#tr[The following image shows a typical installation of the LARUS Vario display in the
instrument panel of a glider.]

#figure(
    image("/img/panel.jpg", width: 10cm),
    caption: [#hr[Installation Situation in the Instrument Panel]],
)<panel>

#tr[The display fits into a standard 57 mm recess and is secured with three M3 screws.]

#figure(
    image("/img/knobs-and-screws.jpg", width: 8cm),
    caption: [#hr[Fasteners]],
)<knobs-and-screws>

#tr[For installation, it is necessary to remove the two rotary knobs with a
1.5 mm hex key.]

== #hr[Installation Orientation]

#picnote("/img/pictograph-blue-cloud.svg")[
    #tr[If you intend to replace an existing device with the LARUS
    Vario Display, please check the desired installation position before drilling the
    7.3 mm hole for the rotary encoder, as the display housing is slightly asymmetrical.]
]

#tr[The display can be mounted in the following orientations: 0° / 90° / 180° / 270°. After mounting
the display, its orientation may need to be adjusted in the "Display Rotation" menu.]

== #hr[Initial Commissioning and Functional Test]

#tr[To start up the device, please follow these steps:

+ Please check that the LARUS Vario display is connected as shown in the drawings in  
  @system-configurations. 
+ Switch on LARUS.
+ Please check whether the LARUS Vario displays start up and a yellow or green 
  satellite pictogram is displayed. The Vario pointers should move slightly around the
  zero position.
+ You should now configure the device. All setting options are documented in detail in chapter 
  settings.]

== #hr[Maintenance] 

#tr[The entire system contains no parts that require maintenance. To claim warranty services, 
please contact SteFly directly.]

#picnote("/img/pictograph-yellow-warning.svg")[
    #tr[Opening the housing of the LARUS Vario display will void the warranty.]
]

== #hr[Firmware Update]

#tr[The LARUS team continuously improves the software and releases firmware updates. To
update the firmware, proceed as follows:

+ Switch off the device.
+ Save the new \*.bin file to the SD card included in the scope of delivery and insert 
  it into the SD card slot on the back of the LARUS Vario display.
+ Switch on the device.
+ If firmware is detected on the SD card, the display will remain black for about 3-5 seconds 
  before the message "#keep[Installing... Do NOT power off device]" appears.
+ The device will restart automatically. During the first 10 seconds, the firmware version will be displayed in the Info1 area
 .

The LARUS Vario display only installs compatible firmware versions. If several firmware 
versions are stored on the card, the latest one will be installed.

If the installation fails, please repeat the process. If the
installation fails again, please use a different SD card. The SD card must
be at least 4 GB in size (type SDHC) and formatted with FAT32. The format must be compatible with
DOS/Windows 95 (not GPT).]
