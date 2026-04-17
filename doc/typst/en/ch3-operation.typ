#import "../manual.typ": *

= #hr[Operation]
== #hr[Operation]

#figure(
    image("/img/vario-display.jpg", height: 6cm),
    caption: [#hr[LARUS Vario Display]],
)<vario-display>

#tr[The device features a two-stage rotary knob and a push button. The following functions are assigned to the
two rotary knobs:

- Turn the small/upper rotary knob: Volume control
- Turn the large/lower rotary knob: MacCready value
- Press the rotary knob briefly: Flight Menu
- Press and hold the rotary knob: Settings Menu

In the menus, the desired item can be selected using the rotary knobs. Selection is confirmed by 
a short press of the button. The menus contain either submenus or configuration parameters to choose from. 
Submenus are navigated in the same way. Once a configuration parameter has been selected, 
it can be changed. The changes take effect immediately. A short press of the button returns you to 
the calling menu.

Whether you are in a menu, a submenu or the editor: you can always return directly to the basic display by 
pressing and holding the button.

The device provides three basic displays: Vario, Horizon and Device Information. 
You can switch between the displays by selecting the ‘Display’ option in the Flight 
Menu after a short press of the button. Alternatively, whilst holding down the rotary knob, you can select the desired 
basic display by turning it. The selected display remains active permanently. 
The various display modes are described in the following sections. ]

== Vario Display
<vario-display>

#tr[The central display distinguishes between circling mode and straight flight mode. The 
switch between these two modes is automatic. If a rotation speed of 
at least 1°/sec is measured for more than 10 seconds, circling mode is activated. If 
the rotation speed falls below this value for more than 10 seconds, the system switches back to straight flight mode.]

#picnote("/img/pictograph-blue-cloud.svg")[
  #tr[Switching from variometer mode to mode to fly does not affect the information displayed on the main screen.]
]

=== #hr[Circling Mode]

#figure(
    image("/img/circling-explained.svg", width: 15cm),
    caption: [#hr[Display in Circling Mode]],
)<circling-explained>

#tr[The Info 1, Info 2, Info 3 displays and the central display depend on whether you are flying straight ahead
or in circling mode. The content of these displays can be customised for both modes.

Content in circling mode:

- Current climb rate
- Average climb rate
- MacCready value
- Central display, here: thermal assistant
- Info 1 display, here: time
- Info 2 display, here: wind direction and wind speed
- Info 3 display, here: average climb rate since the start of circling
- Symbol area: The colour of the Sat symbol indicates data quality:
  - Green: Connection to the LARUS sensor unit established. Device has a GPS fix.
  -    Yellow: Connection to the LARUS sensor unit established, unit has no GPS fix
  - Red: No connection to the LARUS sensor unit
- Icon area: The colour of the battery icon corresponds to the operating voltage:
  - Green: Battery voltage is sufficient.
  - Yellow: The battery voltage is in the critical range.
  - Red: The battery voltage is below the minimum value.
- Icon area: Circle: Usage Mode Club, User Profile 1 (Usage Mode Normal as a square). ]

=== #hr[Straight Flight Mode]

#figure(
    image("/img/straight-explained.svg", width: 15cm),
    caption: [#hr[Display in Straight Flight Mode]],
)<straight-explained>

#tr[Display in straight flight mode:

- Central display: shows wind direction relative to the aircraft’s longitudinal axis
- Average climb rate
- MacCready value
- Speed indicator: The position of the bar indicates whether you are flying too fast or too slow. 
  Positive values mean you are flying too fast; negative values mean you are 
  flying too slow. The length of the bar indicates by how much. 1 m/s corresponds to 10 km/h.
- Symbol area: Battery symbol OK, Sat symbol OK, Usage Mode Club, User Profile 1
- Info 1 display, showing wind angle
- Info 2 display, showing wind direction and wind speed
- Info 3 display, showing speed to fly ]

=== #hr[Warnings]

#figure(
    image("/img/warning.svg", height: 38mm),
    caption: [#hr[Displaying a Warning and Identifying the Cause]],
)<straight-explained>

#tr[The LARUS Vario display alerts the user if it encounters problems processing data. A red warning triangle with an exclamation mark will then appear. Although the Vario is still functioning correctly, the display quality may be reduced.

Possible causes:

- GNSS reception may be (temporarily) restricted.
- The magnetic sensor may be malfunctioning.

The cause of the fault can be identified on the ‘Device Info’ page (@device-info). In the example shown, GNSS reception is limited. If this warning appears frequently or continuously on the LARUS Vario display, there is an installation issue (see @trouble-shooting).
]

=== #hr[Available Central Displays]
==== #hr[Key Displays in Circling Mode]

#figure(
    image("/img/circling-single-arrow.png", width: 5cm),
    caption: [#hr[Wind Indicator with an Arrow and a Flag]],
)<circling-single-arrow>

#tr[The current wind direction is indicated by a central arrow. The size of the arrow is
proportional to the wind speed. Changes in wind direction relative to the average direction
are indicated by a wind vane; changes in wind speed relative to the
average speed over the medium term are indicated by the width of the wind vane. The
direction of the arrow is relative to north, symbolised by the letter N on the scale at the top.]

#figure(
    image("/img/circling-double-arrow.png", width: 5cm),
    caption: [#hr[Wind Indicator with Two Arrows]],
)<circling-double-arrow>

#tr[The current wind direction and speed are indicated by the blue arrow,
while information on the average wind is shown by the grey arrow in the background. The
size of the arrow depends on the wind speed. The direction of the arrow is relative to north.]

#figure(
    image("/img/circling-dotted-assistant.png", width: 5cm),
    caption: [#hr[Centring Aid with Dots]],
)<circling-dotted-assistant>

#tr[The Thermal Assistant can help pilots centre themselves within a thermal. It clearly shows
where good and less favourable climb rates can be found. This information is particularly
useful, as the LARUS system displays climb rates in real time. 

Meaning of the colours of the circular dots:

- Yellow: Maximum climb rate
- Black: Minimum climb rate
- Red: Climb rate is above average
- Blue: Climb rate is below average 

The diameter of the dots is proportional to the climb rate. A steady updraft is 
optimally centred when blue and red dots occur with roughly equal frequency.]

#figure(
    image("/img/circling-spider-assistant.png", width: 5cm),
    caption: [#hr[Centring Aid with Spider's Web]],
)<circling-spider-assistant>

#tr[Meaning of the colours of the circular segments:

- Yellow: Maximum climb rate
- Black: Minimum climb rate
- Red: Climb rate is above average
- Blue: Climb rate is below average

The diameter of the segment is proportional to the climb rate. A steady updraft
is optimally centred when blue and red areas appear with equal frequency.]

==== #hr[Central Displays in Straight Flight Mode]

#figure(
    image("/img/straight-single-arrow.png", width: 5cm),
    caption: [#hr[Wind Indicator for Straight-Ahead Navigation, Featuring an Arrow and a Flag]],
)<straight-single-arrow>

#tr[The current wind direction is indicated by a central arrow. The size of the arrow is
proportional to the wind speed. Changes in wind direction relative to the average direction
are indicated by a wind vane, whilst changes in wind speed relative to the
average speed over a medium period are indicated by the width of the wind vane. The
aircraft symbol indicates that the display refers to the direction of flight.]

#figure(
    image("/img/straight-double-arrow.png", width: 5cm),
    caption: [#hr[Wind Indicator for Straight-Ahead Navigation with Two Arrows]],
)<straight-double-arrow>

#tr[The current wind direction and speed are indicated by the blue arrow,
while information on the average wind is shown by the grey arrow in the background. The
size of the arrows depends on the wind speed. The direction of the arrows is relative to the
longitudinal axis of the glider. The glider symbol indicates that the display relates to the
direction of flight.]

== #hr[Artificial Horizon]

#picnote("/img/pictograph-yellow-warning.svg")[
    #tr[This is not an officially approved artificial horizon. Therefore, this display must not 
    be used for flying in clouds or otherwise outside VFR conditions.]
]

#figure(
    image("/img/horizon.png", width: 5cm),
    caption: [#hr[Artificial Horizon]],
)<horizon>

#tr[The artificial horizon display shows the following information:

- The blue area represents the sky.
- The boundary with the brown area corresponds to the horizon.
- The circular scale at the top shows the glider’s current
      bank angle in 15° increments (here approximately 30°, red arrowhead).
- The scale parallel to the horizon indicates the angle of climb rate or descent in 10° increments
      (here 0°).
- An inclinometer is visible in the lower section, which indicates any yaw
      .

The glider is currently in a clean right-hand turn with a bank angle of
30°.]

#figure(
    image("/img/horizon-blocked.png", width: 3cm),
    caption: [#hr[Artificial Horizon Blocked]],
)<horizon-blocked>

#picnote("/img/pictograph-yellow-warning.svg")[
  #tr[In some competitions, the display of the artificial
  horizon is prohibited. For this reason, the output of
  horizon information can be disabled in the LARUS sensor unit. In this case, a warning is displayed
  instead of the horizon.]
]
  
== #hr[Device Information]<device-info>

#tr[In the Device Information display mode, file information regarding the LARUS Vario Display and
the LARUS sensor unit is shown. This display can be useful for carrying out fault analyses
or retrieving specific information. For example, all the states
of the inputs and outputs can be viewed here. Fault conditions of the LARUS sensor unit
can also be identified.]

#figure(
    image("/img/device-info.png", width: 5cm),
    caption: [#hr[Device Information]],
)

== Flight Menu
<flight_menu>

#tr[The #keep[Flight Menu] provides settings that are required before or during
flight. The menu can be accessed by briefly pressing the control button. The headings in this section are the same as those used in the device.]

=== Water Ballast
#tr[This field specifies the amount of water ballast loaded. When draining the water during
flight, this value can be adjusted manually or reduced automatically via a switch on the valve and
the corresponding configuration.

The set value is synchronised with a connected navigation computer.]

=== Bugs
#tr[Insects on the wings and fuselage reduce the glider’s glide performance. These
changes in a glider’s performance can be approximated using this setting.
Settings ranging from 0 to 50 per cent are possible. At a setting of
50 per cent, the rate of descent doubles at a given speed.] 

Der Algorithmus arbeit exakt identisch zu XCSoar / OpenSoar. Der eingestellte Wert wird zu einem
angeschlossenen Navigationsrechner synchronisiert.

=== Pilot Weight
#tr[The pilot’s weight is taken into account when calculating the glider’s polar curve. For two-seaters,
the combined weight of both pilots must be entered here

The value entered is synchronised with a connected navigation computer.]

=== Display
#tr[The 'Display' menu option allows you to select what is shown on the device’s screen at all times. Here, you can switch between
Vario, Horizon and device information.]

=== User Profile<user-profile>
#tr[The LARUS Vario Display offers a wide range of settings to customise the display to the pilot’s
needs. If several pilots fly the same glider, the 
#keep[User Profiles] feature allows for easy switching between the different settings. Up to 
4 different profiles can be used.

As described in the ‘Usage Mode’ section, whether 3 or
4 user profiles are available. Some settings are standardised across all four user profiles
as they depend on the glider and the installation configuration. This applies, for example, to the
glider’s polar curve and hardware pin configurations. This ensures that these
settings are available in all profiles.]

