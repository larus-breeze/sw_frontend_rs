#import "../manual.typ": *

= #hr[Operation]
== #hr[Operation]

#figure(
    image("/img/vario-display.jpg", height: 6cm),
    caption: [#hr[LARUS Vario Display]],
)<vario-display>

#tr[The device has a rotary knob with two levels and a push button. The following functions are assigned to the button and the
two rotary knobs:

- Turn the small/upper rotary knob: Volume control
- Turn the large/lower rotary knob: MacCready value
- Short press on the rotary knob: Flight Menu
- Press and hold the rotary knob: Settings Menu

The device provides three basic displays: Vario, Horizon and Device Information.
You can switch between the displays by briefly pressing the button and selecting the
Display item in the menu. The selected display remains permanently. The following
sections describe the different display modes. ]

== Vario Display
<vario-display>

#tr[The central display distinguishes between circling mode and straight flight mode. Switching between these two modes is automatic. If a rotation speed of at least 1°/sec is measured for more than 10 seconds, circling mode is activated. If the rotation speed is below this value for more than 10 seconds, the device switches back to straight flight mode.]

#picnote("/img/pictograph-blue-cloud.svg")[
  #tr[The change from variometer mode to speed to fly mode is independent of the change in the central display information.]
]

=== #hr[Circling Mode]

#figure(
    image("/img/circling-explained.svg", width: 15cm),
    caption: [#hr[Display in Circling Mode]],
)<circling-explained>

#tr[The displays Info 1, Info 2, Info 3 and the central display depend on whether you are flying straight ahead
or circling. The content of these displays can be set for both modes.

Content in circling mode:

- Current climb rate
- Average climb rate
- MacCready value
- Central display, here thermal assistant
- Info 1 display, here time
- Info 2 display, here wind direction and wind strength
- Info 3 display, here average climb rate since the start of circling
- Symbol area: The colour of the Sat symbol indicates the data quality:
  - Green: Connection to the LARUS sensor unit established. Device has GPS fix.
  - Yellow: Connection to LARUS sensor unit established, unit does not have GPS fix
- Red: No connection to LARUS sensor unit
- Symbol area: The colour of the battery symbol corresponds to the operating voltage:
- Green: Battery voltage is sufficient.
  - Yellow: The battery voltage is in the critical range.
  - Red: The battery voltage is below the minimum value.
- Symbol area: Circle: Usage Mode Club, User Profile 1 (Usage Mode Normal as a square). ]

=== #hr[Straight Flight Mode]

#figure(
    image("/img/straight-explained.svg", width: 15cm),
    caption: [#hr[Display in Straight Flight Mode]],
)<straight-explained>

#tr[Content in straight flight mode:

- Central display, here wind direction in relation to aircraft longitudinal axis
- Average climb rate
- MacCready value
- Target speed indicator: The position of the bar indicates whether you are flying too fast or too slow. 
  Positive values mean that you are flying too fast, negative values mean that you are flying too 
  slowly. The length of the bar indicates by how much. 1 m/s corresponds to 10 km/h.
- Symbol area: Battery symbol OK, Sat symbol OK, Usage Mode Club, User Profile 1
- Info 1 display, here wind offset angle
- Info 2 display, here wind direction and wind speed
- Info 3 display, here speed to fly ]

=== #hr[Available Central Displays]
==== #hr[Central Displays in Circling Mode]

#figure(
    image("/img/circling-single-arrow.png", width: 5cm),
    caption: [#hr[Wind Indicator with an Arrow and Flag]],
)<circling-single-arrow>

#tr[The current wind direction is indicated by a central arrow. The size of the arrow is
proportional to the wind speed. Changes in wind direction relative to the average direction
are indicated by a wind vane, while changes in wind speed relative to the
medium-term average speed are indicated by the width of the wind vane. The
arrow direction refers to north, symbolised by the N on the scale above.]

#figure(
    image("/img/circling-double-arrow.png", width: 5cm),
    caption: [#hr[Wind Indicator with Two Arrows]],
)<circling-double-arrow>

#tr[The current wind direction and speed are indicated by the blue arrow,
information about the average wind is shown by the grey arrow in the background. The
size of the arrow depends on the wind speed. The direction of the arrow refers to north.]

#figure(
    image("/img/circling-dotted-assistant.png", width: 5cm),
    caption: [#hr[Centring Aid with Dots]],
)<circling-dotted-assistant>

#tr[The Thermic Assistant can help pilots centre the thermals. It clearly shows
where good and less good climb rate can be found. This information is particularly
useful as the LARUS system displays the climb rate without delay. 

Meaning of the colours of the circle points:

- Yellow: Maximum climb rate
- Red: Climb rate is above average
- Blue: Climb rate is below average 

The diameter of the points is proportional to the climb rate. A constant updraft is 
optimally centred when blue and red points occur with approximately equal frequency.]

#figure(
    image("/img/circling-spider-assistant.png", width: 5cm),
    caption: [#hr[Centring Aid with Spider Web]],
)<circling-spider-assistant>

#tr[Meaning of the colours of the circle segments:

- Yellow: Maximum climb
- Red: Climb is above average
- Blue: Climb is below average

The diameter of the segment is proportional to the climb rate. A constant updraft
is optimally centred when blue and red areas appear with equal frequency.]

==== #hr[Central Displays in Straight Flight Mode]

#figure(
    image("/img/straight-single-arrow.png", width: 5cm),
    caption: [#hr[Wind Indicator for Straight Flight with an Arrow and Flag]],
)<straight-single-arrow>

#tr[The current wind direction is indicated by a central arrow. The size of the arrow is
proportional to the wind speed. Changes in wind direction relative to the average direction
are indicated by a wind vane, while changes in wind speed relative to the
medium-term average speed are indicated by the width of the wind vane. The
aircraft symbol indicates that the display refers to the flight direction.]

#figure(
    image("/img/straight-double-arrow.png", width: 5cm),
    caption: [#hr[Wind Indicator in Straight Flight with Two Arrows]],
)<straight-double-arrow>

#tr[The current wind direction and speed are indicated by the blue arrow,
while information about the average wind is shown by the grey arrow in the background. The
size of the arrows depends on the wind speed. The direction of the arrows refers to the
longitudinal axis of the glider. The glider symbol indicates that the display refers to the
direction of flight.]

== #hr[Artificial Horizon]

#picnote("/img/pictograph-yellow-warning.svg")[
    #tr[This is not an officially approved artificial horizon. Therefore, this display must not 
    be used to fly in clouds or otherwise outside VFR conditions.]
]

#figure(
    image("/img/horizon.png", width: 5cm),
    caption: [#hr[Artificial Horizon]],
)<horizon>

#tr[The artificial horizon display contains the following information:

- {The blue area represents the sky.
- {The boundary to the brown area corresponds to the horizon.
- {The circular scale at the top shows the current
      bank angle of the glider in 15° increments (here approximately 30°, red arrowhead).
- {The scale parallel to the horizon represents the climb/descent angle in 10° increments
      (here 0°).
- {In the lower area, an inclinometer can be seen, which indicates any possible drift
      .

The glider is currently in a clean right turn with a bank angle of
30°.]

#picnote("/img/pictograph-yellow-warning.svg")[
  #tr[In some competitions, the display of the artificial
  horizon is prohibited. Therefore, the output of horizon information can be blocked in the LARUS sensor unit
 . In this case, a warning is issued instead of the horizon
.]
]
  
== #hr[Device Information]

#tr[The device information display mode shows file information about the LARUS Vario display and
the LARUS sensor unit. This display can be helpful for performing error analyses
or retrieving specific information. For example, all statuses
of the inputs and outputs can be viewed here. Error statuses of the LARUS sensor unit
can also be identified.]

#figure(
    image("/img/device-info.png", width: 5cm),
    caption: [#hr[Device Information]],
)<device-info>

== Flight Menu
<flight_menu>

#tr[The #keep[Flight Menu] provides settings that are required before or during flight.
The menu can be accessed by briefly pressing the control button. The headings in this section are named as they appear in the device.]

=== Water Ballast
#tr[This is where the amount of water ballast to be taken on board is specified. When draining the water during
flight, this value can be corrected manually or reduced automatically by means of a switch on the valve and
the corresponding configuration.

The set value is synchronised with a connected navigation computer.]

=== Bugs
#tr[Insects on the wings and fuselage reduce the gliding performance of the glider. These
changes in the performance of a glider can be approximated with this setting.
 Settings from 0 to 50 percent are possible. At a setting of
50 percent, the sink rate doubles at a given speed.] 

Der Algorithmus arbeit exakt identisch zu XCSoar / OpenSoar. Der eingestellte Wert wird zu einem
angeschlossenen Navigationsrechner synchronisiert.

=== Pilot Weight
#tr[The pilot's weight is taken into account when calculating the glider polar. For two-seaters,
the combined weight of both pilots must be entered here.

The set value is synchronised with a connected navigation computer.]

=== Display
#tr[The Display menu item allows you to select the permanent display of the device. Here you can switch between
Vario, Horizon and Device Information.]

=== User Profile<user-profile>
#tr[The LARUS Vario Display offers many setting options to adapt the displays to the pilot's needs.
 If several pilots fly a glider, the 
#keep[User Profiles] allow convenient switching between the different settings. Up to 
4 different profiles can be used.

As described in the Usage Mode section, whether 3 or
4 usage profiles are available. Some settings are synchronised in all four usage profiles,
 as they depend on the glider and the installation situation. This applies, for example, to the
glider's polars and hardware pin configurations. This ensures that these
settings are available in all profiles.]

