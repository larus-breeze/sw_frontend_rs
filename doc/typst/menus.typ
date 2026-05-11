#import "manual.typ": *

= Menus

#set par(leading: 1.35mm)
#set text(size: 8pt)

Flight Menu (@flight_menu) \
├── Water Ballast \
├── Bugs \
├── Pilot Weight \
├── Display \
├── User Profile \
└── Return \


Settings (@settings) \
├── Views (@views) \
│#h(8mm)├── Circling \
│#h(8mm)│#h(8mm)├── Center Content \
│#h(8mm)│#h(8mm)├── Info 1 Content \
│#h(8mm)│#h(8mm)├── Info 2 Content \
│#h(8mm)│#h(8mm)├── Info 3 Content \
│#h(8mm)│#h(8mm)└── Return \
│#h(8mm)│#h(8mm) \
│#h(8mm)├── Straight \
│#h(8mm)│#h(8mm)├── Center Content \
│#h(8mm)│#h(8mm)├── Info 1 Content \
│#h(8mm)│#h(8mm)├── Info 2 Content \
│#h(8mm)│#h(8mm)├── Info 3 Content \
│#h(8mm)│#h(8mm)└── Return \
│#h(8mm)│#h(8mm) \
│#h(8mm)├── Units \
│#h(8mm)│#h(8mm)├── Horizontal Speed \
│#h(8mm)│#h(8mm)├── Vertical Speed \
│#h(8mm)│#h(8mm)├── Height \
│#h(8mm)│#h(8mm)└── Return \
│#h(8mm)│#h(8mm) \
│#h(8mm)├── Block Horizon \
│#h(8mm)├── Energy Arrow \
│#h(8mm)├── Display Rotation \
│#h(8mm)├── Glider Symbol \
│#h(8mm)└── Return \
│#h(8mm) \
├── Advanced (@advanced) \
│#h(8mm)├── User Profiles \
│#h(8mm)│#h(8mm)├── Usage Mode \
│#h(8mm)│#h(8mm)├── Code \
│#h(8mm)│#h(8mm)├── Config Reset \
│#h(8mm)│#h(8mm)├── Factory Reset \
│#h(8mm)│#h(8mm)└── Return \
│#h(8mm)│#h(8mm) \
│#h(8mm)├── Vario \
│#h(8mm)│#h(8mm)├── Avg Climb Source \
│#h(8mm)│#h(8mm)├── TC Climb Rate \
│#h(8mm)│#h(8mm)├── Vario Upper Limit \
│#h(8mm)│#h(8mm)├── Vario Lower Limit \
│#h(8mm)│#h(8mm)└── Return \
│#h(8mm)│#h(8mm) \
│#h(8mm)├── Speed to Fly \
│#h(8mm)│#h(8mm)├── TC Circle Hyst \
│#h(8mm)│#h(8mm)├── TC Speed to Fly \
│#h(8mm)│#h(8mm)├── Vario Control \
│#h(8mm)│#h(8mm)├── StF Pin Config \
│#h(8mm)│#h(8mm)├── StF Upper Limit \
│#h(8mm)│#h(8mm)├── StF Lower Limit \
│#h(8mm)│#h(8mm)└── Return \
│#h(8mm)│#h(8mm) \
│#h(8mm)├── Gear Alarm \
│#h(8mm)│#h(8mm)├── Alarm Volume \
│#h(8mm)│#h(8mm)├── Gear Alarm Config \
│#h(8mm)│#h(8mm)├── Gear Pin Config \
│#h(8mm)│#h(8mm)├── Airbrakes Pin Config \
│#h(8mm)│#h(8mm)└── Return \
│#h(8mm)│#h(8mm) \
│#h(8mm)├── Drain Control \
│#h(8mm)│#h(8mm)├── Drain Pin Config \
│#h(8mm)│#h(8mm)├── Flow \
│#h(8mm)│#h(8mm)└── Return \
│#h(8mm)│#h(8mm) \
│#h(8mm)├── Flash Control \
│#h(8mm)│#h(8mm)├── Flash Control \
│#h(8mm)│#h(8mm)├── Flash Test \
│#h(8mm)│#h(8mm)└── Return \
│#h(8mm)│#h(8mm) \
│#h(8mm)├── Sound \
│#h(8mm)│#h(8mm)├── Center Frequency \
│#h(8mm)│#h(8mm)├── Waveform \
│#h(8mm)│#h(8mm)├── Spreading Factor \
│#h(8mm)│#h(8mm)└── Return \
│#h(8mm)│#h(8mm) \
│#h(8mm)├── More Settings \
│#h(8mm)│#h(8mm)├── Battery Good \
│#h(8mm)│#h(8mm)├── Battery Low \
│#h(8mm)│#h(8mm)└── Return \
│#h(8mm)│#h(8mm) \
│#h(8mm)└── Return \
│#h(8mm) \
├── Polar Settings (@polar-settings) \
│#h(8mm)├── Glider \
│#h(8mm)├── Empty Mass \
│#h(8mm)├── Max Ballast \
│#h(8mm)├── Reference Weight \
│#h(8mm)├── Polar V 1 \
│#h(8mm)├── Polar V 2 \
│#h(8mm)├── Polar V 3 \
│#h(8mm)├── Polar Si 1 \
│#h(8mm)├── Polar Si 2 \
│#h(8mm)├── Polar Si 3 \
│#h(8mm)└── Return \
│#h(8mm) \
├── Sensor Box (@sensor-box) \
│#h(8mm)├── Left Wing Down \
│#h(8mm)├── Right Wing Down \
│#h(8mm)├── Wings Straight \
│#h(8mm)├── Calc Orientation \
│#h(8mm)├── Straight Flight \
│#h(8mm)├── Reset Sensorbox \
│#h(8mm)├── Init Settings \
│#h(8mm)│#h(8mm)├── Sensor Tilt Roll \
│#h(8mm)│#h(8mm)├── Sensor Tilt Pitch \
│#h(8mm)│#h(8mm)├── Sensor Tilt Yaw \
│#h(8mm)│#h(8mm)├── Pitot Offset \
│#h(8mm)│#h(8mm)├── Pitot Span \
│#h(8mm)│#h(8mm)├── QNH Delta \
│#h(8mm)│#h(8mm)├── Vario TC \
│#h(8mm)│#h(8mm)├── GNSS Config \
│#h(8mm)│#h(8mm)├── Ant Base Len \
│#h(8mm)│#h(8mm)├── Ant Slave Down \
│#h(8mm)│#h(8mm)├── Ant Slave Right \
│#h(8mm)│#h(8mm)└── Return \
│#h(8mm)│#h(8mm) \
│#h(8mm)├── Test Function \
│#h(8mm)│#h(8mm)├── Test Parameter \
│#h(8mm)│#h(8mm)├── Test Function \
│#h(8mm)│#h(8mm)└── Return \
│#h(8mm)│#h(8mm) \
│#h(8mm)└── Return \
│#h(8mm) \
└── Return \

