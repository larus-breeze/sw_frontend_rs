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
            [#tr[Housing dimensions without knob]],[60,5 mm x 63,4 mm x 31,2 mm],
            [#tr[input voltage]],[9 - 28 V DC],
            [#tr[current consumption]],[80 mA],
            [#tr[interfaces]],[RS232-Rj45, CAN-Rj45, 4-Input, 2-Output-DSub9, Micro-SD, 3,5 mm Audio],
            [#tr[NMEA interface]], [RS232 38400 Baud, #link("https://github.com/larus-breeze/doc_larus/blob/master/documentation/Larus_NMEA_Protocol.md")[#tr[specification]]],
            [#tr[CAN interface]],[1 MBAud, #link("https://github.com/larus-breeze/doc_larus/blob/master/documentation/can_spec.md")[#tr[specification]]],
            [#tr[temperatures]],[-30°C ... 60°C],
            [#tr[moisture]],[0\% - 90 \%],
            [#tr[case material]],[#tr[Black anodised aluminium]],
            table.hline(),
        ),
        caption: [#hr[CAN and RS232 RJ45 Pin Assignment]],
    )
])
#pagebreak()