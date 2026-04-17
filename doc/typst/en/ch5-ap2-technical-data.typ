#import "../manual.typ": *

= #hr[Technical Specifications]

#h(5mm)
#align(left, block[
    #figure(
        table(
            columns: (auto, auto),
            align: (left, left),
            table.hline(),
            [#tr[Weight]], [80g],
            [#tr[Housing dimensions excluding the knob]],[60,5 mm x 63,4 mm x 31,2 mm],
            [#tr[Input voltage]],[9 - 28 V DC],
            [#tr[Power consumption]],[80 mA],
            [#tr[Interfaces]],[RS232-Rj45, CAN-Rj45, 4-Input, 2-Output-DSub9, Micro-SD, 3,5 mm Audio],
            [#tr[NMEA interface]], [RS232 38400 Baud, #link("https://github.com/larus-breeze/doc_larus/blob/master/documentation/Larus_NMEA_Protocol.md")[#tr[Specification]]],
            [#tr[CAN interface]],[1 MBAud, #link("https://github.com/larus-breeze/doc_larus/blob/master/documentation/can_spec.md")[#tr[Specification]]],
            [#tr[Temperatures]],[-30°C ... 60°C],
            [#tr[Humidity]],[0\% - 90 \%],
            [#tr[Housing material]],[#tr[Black anodised aluminium]],
            table.hline(),
        ),
        caption: [#hr[Pin Configuration for CAN and RS232 RJ45]],
    )
])
#pagebreak()