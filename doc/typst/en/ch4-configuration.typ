#import "../manual.typ": *

= Settings
<settings>

#tr[The functions stored here are used to configure the system. The headings
 correspond to those in the English texts in the LARUS Vario Display menus to make them easier to find
.]

== Views
<views>
=== Circling, Straight
#tr[These two menu items determine what is displayed during straight flight and what is displayed during circular flight.
The switch between circular flight and straight flight is automatic.

The following views can be set for each:

- #keep[*Center Content:*] Centre display
- #keep[*Info 1 Content:*] Top line
- #keep[*Info 2 Content:*] Bottom line
- #keep[*Info 3 Content:*] Right margin
]

=== Units

#tr[Various displays are shown using a unit of measurement. Here you can specify
which units of measurement are to be used:

- #keep[*Horizontal Speed*] Unit of measurement for horizontal speed
- #keep[*Vertical Speed*] Unit of measurement for vertical speed
- #keep[*Height*] Unit of measurement for altitude display
]

=== Energy Arrow

#tr[The energy arrow is intended to show the direction in which an increasing climb rate is likely to occur. It displays the vectorial difference between the current and average measured wind.]

=== Display Rotation

#tr[The display can be installed at different angles (0°, 90°, 180° or 270°). The
display can be adjusted to suit the installation situation.]

=== Glider Symbol

#tr[When flying straight ahead, the wind indicator refers to the longitudinal axis of the aircraft. This can be symbolised by
displaying the floor plan of a glider. The symbolism can be activated
or deactivated.]

== Advanced
<advanced>
=== User Profiles
==== Usage Mode, Code
<usage-modes>

#tr[The LARUS Vario Display's settings allow for individual configurations according to the
pilot's needs. In club operations, it is therefore not uncommon for irritation to arise
when a pilot encounters a device with unfamiliar settings. For this reason, the LARUS
Vario Display supports two modes: Normal and Club.

In Normal mode (default setting), all settings can be adjusted as desired. There are
four user profiles (0..3) available in this mode. This allows up to four pilots to permanently
use different settings.

In Club mode, two opposing goals are pursued. On the one hand, pilots should be able to use
useful settings, but on the other hand, standardised settings should be
provided. To achieve this, profile 0 is locked. This profile serves as a
template for the standardised settings. Profiles 1, 2 and 3 can be used as usual. However, some configuration points, such as polar settings, hardware pin assignments, or access to the sensor unit, are excluded. Profile 1 is reset to the default values and activated on each new flight day. The menu item "#keep[User Profile]" (@user-profile) 
also provides a function to reset the selected profile to the default values if necessary.

Switching between "#keep[Usage Modes]" is secured by a code.
The code is derived from the firmware version. For example, firmware v0.3.8.56 expects
the code 3856.]

==== Config Reset
#tr[This function can be used to reset the currently selected user profile to the default values. The default values are "hard-coded" in the device and cannot be changed. All settings relating to the displayed data are reset. Settings relating to the aircraft or hardware are retained.

This function is also available in "#keep[Usage Mode]" Normal and should not be confused with resetting
to default values from profile 0 in "#keep[Usage Mode]" Club.]

==== Factory Reset
#tr[This resets the device to its factory settings. This affects 
all settings for all profiles.]

=== Vario
==== Avg Climb Source, TC Climb Source

#tr[Two different sources are supported for determining the average climb rate. The differences
are as follows: Avg Climb Source:

- *Front end:* The average is calculated during circling. When switching
  from speed to fly to vario, the current vario value is used as the start value. When switching
  from vario to speed to fly, averaging is stopped and the display remains constant.
  The time constant for averaging can be adjusted.
- *Sensor box:* Averaging is performed continuously. During straight flight,
  averaging is performed with a fixed time constant, which can be set in the sensor box menu.
  During circling, averaging is performed synchronously with circling.
]

==== Vario Upper Limit, Vario Lower Limit

#tr[The acoustic signal is muted between these two values.]

=== Speed to Fly
==== TC Circle Hyst

#tr[The hysteresis, i.e. the waiting time when switching between vario and speed to fly, is set here.]

==== TC Speet to Fly}

#tr[The display of the speed to fly is damped so as not to irritate the pilot with a nervous display. Here, you can specify the time constant with which this damping should take place.]

==== Vario Control, StF Pin Config

#tr[The LARUS Vario Display supports various methods for switching between the vario and
speed to fly displays. The following options are available:

- *Auto:* Switching depends on the airspeed. The limit
          is 1.1 times the speed for the best glide. When setting the
          limit, the aircraft polar curve and the load (pilot weight, water ballast) are taken into account.
          During circling, there is no switchback to Vario.
- *Input Pin:* The switchover is triggered by a switch or button (selectable).
          The hardware configuration must be configured additionally (switch/button The hardware configuration must also be configured (switch/button
          and polarity) \textbf{StF Pin Config.
- *NMEA:* The switchover is triggered by XCSoar/OpenSoar. This setting
          can also be used if a stick remote control for XCSoar/OpenSoar with a speed to fly button is used.
- *CAN:* In two-seater installations, it may be desirable for the
          speed to fly/vario switchover to be triggered by the second display device. If this
          second device performs the switchover automatically, for example, this ensures
          that both displays work in synchronisation.
]
==== Stf Upper Limit, Stf Lower Limit

#tr[You can set a speed range in which the acoustic speed to fly signal
is muted. In the factory setting, the audio signal is deactivated in a range of +/- 10 km/h. This range can be adjusted here to suit individual preferences.]

=== Gear Alarm

#tr[The landing gear warning is designed to remind the pilot to extend the landing gear if he forgets to do so before landing.
The landing gear warning is based on two switches that monitor the airbrakes and landing gear.
The warning is both visual on the display and audible. The warning is both visual on the display and audible. The
switches can be connected directly to the LARAUS Vario display or in series via a
signal line.

Direct connection of both switches to the LARUS Vario display: Both pins must be set up correctly
: #keep[*Gear Pin Config*, *Airbrakes Pin Config*. *Gear Alarm
Config*] must then be set to #keep[*Two Pin Mode*].

Connections from the switches in series: The common line is set up with 
#keep[*GearPinConfig*]. #keep[*Gear Alarm Config*] must then be set to #keep[*One Pin Mode*].

#keep[*Alarm Volume*] allows the volume of an alarm to be adjusted.
]

=== Drain Control

#tr[The switch that monitors the water drainage device is set up with #keep[*Drain Pin Config*].


A constant flow rate is assumed, which must be specified here:] *Flow*. 

=== More Settings

#tr[This section summarises the following settings:

- *Battery Good:* Above the limit value set here, the
              power supply is OK (green battery symbol).
- *Battery Low:* Below the voltage specified here, the battery symbol is displayed in red. If the voltage is between the two values, the battery symbol is displayed in orange.
- *Flash Control:* The LARUS Vario display is capable of controlling a canopy flasher which can be configured here.
]

=== Center Frequency
#tr[Here you can set the centre frequency of the variometer.]

== Polar Settings
<polar-settings>

#tr[To obtain the correct speed to fly information, you must set the correct polar values for your glider type. The LARUS Vario display comes factory-equipped with more than 200 polars from different gliders.

If you cannot find your glider type in the list, you can select any
glider polar and change the individual settings to the values of your
glider polar.]

=== Glider

#tr[Select the correct or closest polar. The name of the aircraft type cannot be
changed.]

#picnote("/img/pictograph-yellow-warning.svg")[
      #tr[Selecting an aircraft type overrides all
        subsequent settings such as empty weight, maximum water ballast, etc. This cannot be
        reversed, even if the identical type is selected again later. All
        specific values must then be re-entered.]
]

=== Empty Mass

#tr[After selecting the glider type, you should adjust the empty weight (without the pilot's weight)
of your glider so that the calculations can be performed correctly.]

=== Max Ballast
#picnote("/img/pictograph-yellow-warning.svg")[
      #tr[Ensure that the maximum water ballast matches the
    specifications of XCSoar/OpenSoar, otherwise the water ballast calibration will not
    function correctly.]
]
    
=== Reference Weight

#tr[The sink rates to the polar curve given below refer to a glider with
the reference mass specified here.]

=== Polar v1, v2, v3, si1, si2, si3

#tr[The speeds and sink rates describe the performance of the glider used. As usual, the polar is represented by a quadratic equation. The speed range in which the glider flies between updrafts is important so that the speed to fly can be calculated correctly.]

== Sensor Box
<sensor-box>

=== #hr[Calibration of the LARUS Sensor Unit]
<sensorunit-calibration>

#tr[Before you begin your first flight, the position sensors of the LARUS sensor unit must be precisely
adjusted. The calibration steps are carried out using a simple procedure, which is
described below and initiated via functions in the LARUS Vario display. The
calibration is carried out in two stages.]

=== #hr[Initial Calibration on the Ground:]

#tr[Assemble your glider and place it on a flat surface. After you have taken up the
individual positions, wait until there are no more vibrations in the aircraft
before continuing with the calibration. Do not use a tail trolley to fix the vertical
axis of your glider during the following procedures.

- *Left Wing Down:* Lower the left wing, wait briefly, and call up the function.
- *Right Wing Down:* Lower the right wing, wait briefly, and call up the function.
- *Wings Straight:* Hold the wing horizontally, wait, call up the function.
- *Calc Orientation:* For this step, it is important to perform all three of the
            above steps. The order of the steps does not matter,
            but they must be completed in full.
]

=== #hr[Fine Adjustment in the Air:]

#tr[Precise pitch angle calibration is performed during flight. It is recommended that
 this step be performed during a flight that is not disturbed by thermal gusts. Align
 your glider at the speed with the best glide ratio (if you have flaps,
 set them to this speed). Call up #keep[*Straight Flight*].
 That's it. You can check the calibration by switching to the
 artificial horizon display (#keep[@horizon]).]

 === Reset Sensorbox
 
 #tr[This function triggers a restart of the sensor box.]

 === Init Settings
 
 #tr[These setting options for the LARUS sensor unit are reserved for experts and will not be described in detail here.]