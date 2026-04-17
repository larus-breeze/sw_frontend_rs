#import "../manual.typ": *

= Settings
<settings>

#tr[The functions listed here are used to configure the system. The headings
correspond to those in the English text in the LARUS Vario Display menus, to make them easier to
find.]

== Views
<views>
=== Circling, Straight
#tr[These two menu options determine what is displayed during straight flight and what is displayed during circling flight.
The switch between circling flight and straight flight occurs automatically.

The following views can be selected:

- #keep[*Center Content:*] Centre display
- #keep[*Info 1 Content:*] Top row
- #keep[*Info 2 Content:*] Bottom row
- #keep[*Info 3 Content:*] Right-hand margin

Various pieces of information can be displayed in these views. The following
is a list of which information can be displayed in which position:

#keep[*Center Content*]:
- #keep[*Single Arrow*] Single arrow with wind vane
- #keep[*Double Arrow*] Two arrows (wind and average wind)
- #keep[*Dotted Assistant*] Thermal assistant with dots
- #keep[*Spider Assistant*] Thermal assistant in the form of a spider’s web

#keep[*Info 1 Content*]:
- #keep[*None*] Nothing
- #keep[*Avg Climb Rate*] Average climb rate
- #keep[*Battery Voltage*] Battery voltage
- #keep[*Circle Diameter*] Diameter when circling
- #keep[*Circle Max-Min*] Difference between maximum and minimum climb rate
- #keep[*Drift Angle*] Drift angle
- #keep[*Flight Level*] Flight level
- #keep[*G-Load*] Acceleration
- #keep[*Speed to Fly*] Speed to fly
- #keep[*True Air Speed*] True airspeed
- #keep[*True Course*] True heading
- #keep[*UTC Time*] UTC time

#keep[*Info 2 Content*]:
- #keep[*None*] Nothing
- #keep[*Avg Climb Rate*] Average climb rate
- #keep[*Battery Voltage*] Battery voltage
- #keep[*Circle Diameter*] Diameter during circling
- #keep[*Circle Max-Min*] Difference between maximum and minimum climb rate
- #keep[*Drift Angle*] Drift angle
- #keep[*Flight Level*] Flight level
- #keep[*G-Load*] Acceleration
- #keep[*Speed to Fly*] Speed to fly
- #keep[*True Air Speed*] True airspeed
- #keep[*True Course*] True heading
- #keep[*UTC Time*] UTC time
- #keep[*Wind, Avg Wind*] Wind, average wind
- #keep[*Wind and Delta*] Wind and difference from average wind

#keep[*Info 3 Content*]:
- #keep[*None*] None
- #keep[*CLimbing*] Climb rate, averaged over the entire updraft
- #keep[*Speed to Fly*] Speed to fly
]

=== Units

#tr[Various aspects of the display are shown using a unit of measurement. This section specifies
which units of measurement are used:

- #keep[*Horizontal Speed*] Unit of measurement for horizontal speed
- #keep[*Vertical Speed*] Unit of measurement for vertical speed
- #keep[*Height*] Unit of measurement for altitude
]

=== Energy Arrow

#tr[The energy arrow indicates the direction in which a climb rate is likely to occur. It shows the vector difference between the current wind and the average measured wind.

This function is based on the assumption that there is a flow field in the vicinity of a thermal updraft. This influences the horizontal wind near the updraft.]

#figure(
    image("/img/energy-arrow.svg", width: 16cm),
    caption: [#hr[Flow Field of a Thermal Updraft According to Martin Dinges @dinges and Its Effect on the Wind Indicator According to Joe Wurts @wurts]],
)<img-energy-arrow>

#tr[Please note that the direction indicated depends on the altitude at which the glider is flying. At low altitude, the indicator points towards the updraft; at high altitude, however, it points in the opposite direction. At low altitude, the air flows towards the updraft; at high altitude, however, it flows away from the updraft.

However, the indicator can also be significant in other situations, such as when flying along a slope.]

#figure(
    image("/img/energy-arrow.png", width: 5cm),
    caption: [#hr[Displaying the Energy Arrow in Flight]],
)

#tr[The event shown in the example was recorded just above a ridge as the glider flew past an updraft which it was later able to utilise. The display can be adjusted using a factor ranging from 0.0 to 10.0. A value of 0.0 means the #keep[*Energy Arrow*] is not displayed, whilst 10.0 indicates a very large display. Start with a setting of 3.5.

Only modern variometer systems that instantly determine wind speed and direction allow for a meaningful display of the Energy Arrow. This is not possible with conventional variometers.]

=== Display Rotation

#tr[The display can be installed at various angles (0°, 90°, 180° or 270°). The
display can be adjusted to suit the installation situation]

=== Glider Symbol

#tr[When flying straight ahead, the wind indicator is aligned with the aircraft’s longitudinal axis. This can be illustrated by
displaying a plan view of a glider. The visual representation can be enabled
or disabled.]

== Advanced
<advanced>
=== User Profiles
==== Usage Mode, Code
<usage-modes>

#tr[The LARUS Vario Display’s settings allow for custom configurations tailored to the
pilot’s needs. In club settings, it is therefore not uncommon for confusion to arise
when a pilot encounters a device configured in an unfamiliar way. For this reason, the LARUS
Vario Display supports two modes: Normal and Club.

In Normal mode (factory setting), all settings can be adjusted as desired. There are
four user profiles (0–3) available in this mode. This allows up to four pilots to permanently
use different settings.

In Club mode, two contrasting objectives are pursued. On the one hand, pilots should be able to
use useful settings; on the other hand, standardised settings
should be maintained. To achieve this, Profile 0 is locked. This profile serves as a
template for the standardised settings. Profiles 1, 2 and 3 can be used as usual
. However, some configuration points, such as polar curve settings, assignment of
hardware pins or access to the sensor unit. Profile 1 is reset to the default values
and activated on each new flight day. The menu item "#keep[User Profile]" (@user-profile) 
also provides a function to reset the selected profile to the default values if required.

Switching between the "#keep[Usage Modes]" profiles is protected by a code.
The code is derived from the firmware version. For example, firmware v0.3.8.56 expects
the code 3856.]

==== Config Reset
#tr[This function can be used to reset the currently selected user profile to its default values. The default values are hard-coded into the device and cannot be changed. All settings relating to the displayed data will be reset. Settings relating to the aircraft or hardware will remain unchanged.

This function is also available in the "#keep[Usage Mode]" Normal mode and must not be confused with the reset
to default values from profile 0 in the "#keep[Usage Mode]" Club mode.]

==== Factory Reset
#tr[This will reset the device to its factory settings. This applies to 
all settings for all profiles.]

=== Vario
==== Avg Climb Source, TC Climb Source

#tr[Two different sources are supported for calculating the average climb rate. The differences
are as follows *Avg Climb Source:* 

- *Frontend:* Averaging takes place whilst circling. When switching
  from speed to fly to vario, the current vario value is used as the starting value. When switching
  from vario to speed to fly, averaging is paused and the display remains constant.
  The time constant for averaging can be adjusted using #keep[*TC Climb Source*].
- *Sensor box:* Averaging takes place continuously. During straight-line flight,
  averaging is performed using a fixed time constant, which can be set in the sensor box menu.
  During circling, averaging takes place synchronously with the circling.
]

==== Vario Upper Limit, Vario Lower Limit

#tr[The audible signal is muted between these two values.]

=== Speed to Fly
==== TC Circle Hyst

#tr[The hysteresis – i.e. the delay when switching between Vario and the speed to fly – is set here
.]

==== TC Speet to Fly

#tr[The display of the speed to fly is damped so as not to confuse the pilot with a jittery display.
Here, you can specify the time constant to be used for this damping.]

==== Vario Control, StF Pin Config

#tr[The LARUS Vario Display supports various methods for switching between the Vario and
speed to fly displays. The following options are available:

- *Auto:* The display switches depending on airspeed. The threshold
          is set at 1.1 times the airspeed required for optimal glide performance. When determining the
          threshold, the aircraft polar curve and the load (pilot’s weight, water ballast)
          are taken into account. The display does not switch back to Vario whilst circling.
- *Input Pin:* The switch is triggered by a switch or push-button (selectable)
          . The hardware configuration must also be set (switch/button
          and polarity) *StF Pin Config*.
- *NMEA:* The switch is triggered by XCSoar/OpenSoar. This setting
          can also be used if a joystick remote control for XCSoar/OpenSoar with
          a speed to fly button is used.
- *CAN:* In two-seater installations, it may be desirable for the
          switch between speed to fly and Vario to be triggered by the second display unit. If this
          second unit performs the switch automatically, for example, this ensures
          that both displays operate in synchronisation.
]
==== Stf Upper Limit, Stf Lower Limit

#tr[You can set a speed range within which the audible speed warning signal
is muted. By default, the audio signal is deactivated within a range of +/- 10 km/h.
This range can be adjusted here to suit your individual preferences.]

=== Gear Alarm

#tr[The landing gear warning is designed to remind the pilot to extend the landing gear if they forget to do so
before landing. The landing gear warning is based on two switches that monitor the airbrakes and
landing gear. The warning is given both visually on the display and audibly. The
switches can be connected either directly to the LARUS Vario display or in series via a
signal line.

Direct connection of both switches to the LARUS Vario display: Both pins must be correctly
configured: #keep[*Gear Pin Config*, *Airbrakes Pin Config*. *Gear Alarm
Config*] must then be set to #keep[*Two Pin Mode*].

Connections for the switches in series: The common wire is connected to 
#keep[*GearPinConfig*]. #keep[*Gear Alarm Config*] must then be set to #keep[*One Pin Mode*].

#keep[*Alarm Volume*] allows you to adjust the volume of an alarm.
]

=== Drain Control

#tr[The switch that monitors the water drainage system is configured with #keep[*Drain Pin Config*]
.

A constant flow rate is assumed, which must be specified here:] *Flow*. 

=== Flash Control
#tr[The functions relating to the canopy flasher are organised as follows:

- *Flash Control:* The LARUS Vario Display is capable of controlling a canopy flasher that
              is activated when travelling at a speed of over 40 km/h relative to the air. Here, you must 
              specify whether the canopy flasher is activated when the switch is open or 
              closed.
- *Flash Test:* The canopy flasher is activated for 10 seconds. This allows you to check
              whether it is working correctly.
]

=== Sound
#tr[The audio output can be adjusted to suit personal preferences using various settings.

- *Centre Frequency:* Specifies the audio frequency at a climb rate of 0 m/s.
- *Waveform:* Selects the waveform of the audio output. This affects the timbre by increasing or decreasing the amount of harmonics.
- *Spreading Factor:* Specifies the extent to which the frequency changes as the glider climbs or descends. A value of 1.0 means that the range from -5 m/s to +5 m/s is spread across two octaves.]

=== More Settings

#tr[This section summarises the following settings:

- *Battery Good:* Above the threshold set here, the
              power supply is OK (green battery icon).
- *Battery Low:* Below the voltage specified here, the
              battery icon is displayed in red. If the voltage lies between the two values,
              the battery icon is displayed in orange.
]

== Polar Settings
<polar-settings>

#tr[To obtain the correct speed to fly information, you must set the correct polar curve values for
your glider type. The LARUS Vario Display comes pre-loaded with more than 200
polar curves from various gliders.

If you cannot find your glider type in the list, you can select any
glider polar and adjust the individual settings to match the values of your
glider’s polar.]

=== Glider

#tr[Select the correct or nearest polar. The name of the aircraft type cannot
be changed.]

#picnote("/img/pictograph-yellow-warning.svg")[
      #tr[Selecting an aircraft type overrides all
        subsequent settings, such as empty weight, maximum water ballast, etc. This cannot
        be undone, even if the same type is selected again later. All
        specific values must then be re-entered.]
]

=== Empty Mass

#tr[Once you have selected the type of glider, you should adjust the empty weight (excluding the pilot’s weight)
of your glider so that the calculations can be carried out correctly.]

=== Max Ballast
#picnote("/img/pictograph-yellow-warning.svg")[
      #tr[Make sure that the maximum water ballast matches the
    specifications in XCSoar/OpenSoar, otherwise the water ballast adjustment will not
    work correctly.]
]
    
=== Reference Weight

#tr[The glide ratios given below for the polar curve refer to a glider with
the reference mass specified here.]

=== Polar v1, v2, v3, si1, si2, si3

#tr[The airspeeds and glide ratios describe the performance of the glider being used. The
polar curve is, as usual, represented by a quadratic equation. It is important to know the
speed range within which the glider is flown between thermals so that the speed to fly can be
calculated correctly.]

#figure(
    image("/img/replicated-polar.svg", width: 12cm),
    caption: [#hr[ASG 32 Aircraft Polar: Detailed Specifications and Approximations]],
)<replicated-polar>

#tr[The polars of classic aircraft such as the ASW 20 or the LS 3 can be perfectly replicated using a quadratic approximation. With modern aircraft, however, it is important to capture the relevant speed range. Furthermore, the replication of the polars is inaccurate. The example shows the polar curve of the ASG 32  from Alexander Schleicher Flugzeugbau @asg32, which is represented very well in the range from 100 km/h to 180 km/h, but not beyond that. The deviation in the lower range can be disregarded, but in the upper range it should be taken into account. In very good weather, the target speed transmitter will set speeds that are too high.]

== Sensor Box
<sensor-box>

=== #hr[Calibration of the LARUS Sensor Unit]
<sensorunit-calibration>

#tr[Before you take off on your first flight, the attitude sensors on the LARUS sensor unit must be
calibrated accurately. The calibration process is carried out via a simple procedure, which
is described below and initiated via functions on the LARUS Vario display. The
calibration is carried out in two steps.]

=== #hr[Initial Calibration on the Ground:]

#tr[Assemble your glider and place it on a level surface. Once you have
positioned each part correctly, wait until you can no longer feel any vibrations in the glider
before proceeding with the calibration. Do not use a tail trolley to stabilise the vertical
axis of your glider during the following procedures.

- *Left Wing Down:* Lower the left wing, wait briefly and call up the function
- *Right Wing Down:* Lower the right wing, wait briefly and call up the function.
- *Wings Straight:* Hold the wing horizontally, wait, then call up the function.
- *Calc Orientation:* For this step, it is important to have completed all three
            previous steps. The order of the steps does not matter,
            but they must be completed in full.
]

=== #hr[Fine-Tuning in the Air:]

#tr[Precise pitch angle calibration is carried out during flight. It is recommended that
you perform this step during a flight that is not disturbed by thermal gusts. Align
your glider at the speed with the best glide ratio (if you have flaps,
set them to this speed). Call up #keep[*Straight Flight*].
 You are now finished. You can check the calibration by switching to the
 artificial horizon display (#keep[@horizon]).]

 === Reset Sensorbox
 
 #tr[This function triggers a restart of the sensor box.]

 === Init Settings
 
 #tr[These settings for the LARUS sensor unit are intended for use by experts only and will
not be described in detail here.]

 === Test Function

 #tr[This test function provided here is reserved for developers of the Larus Sensor Platform
.]