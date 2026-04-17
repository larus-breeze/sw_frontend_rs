#import "../manual.typ": *

= #hr[Technische Daten]

#h(5mm)
#align(left, block[
    #figure(
        table(
            columns: (auto, auto),
            align: (left, left),
            table.hline(),
            [#tr[Gewicht]], [80g],
            [#tr[Gehäuseabmessungen ohne Knopf]],[60,5 mm x 63,4 mm x 31,2 mm],
            [#tr[Eingangsspannung]],[9 - 28 V DC],
            [#tr[Stromaufnahme]],[80 mA],
            [#tr[Schnittstellen]],[RS232-Rj45, CAN-Rj45, 4-Input, 2-Output-DSub9, Micro-SD, 3,5 mm Audio],
            [#tr[NMEA Schnittstelle]], [RS232 38400 Baud, #link("https://github.com/larus-breeze/doc_larus/blob/master/documentation/Larus_NMEA_Protocol.md")[#tr[Spezifikation]]],
            [#tr[CAN Schnittstelle]],[1 MBAud, #link("https://github.com/larus-breeze/doc_larus/blob/master/documentation/can_spec.md")[#tr[Spezifikation]]],
            [#tr[Temperaturen]],[-30°C ... 60°C],
            [#tr[Feuchtigkeit]],[0\% - 90 \%],
            [#tr[Gehäusematerial]],[#tr[Schwarz eloxiertes Aluminium]],
            table.hline(),
        ),
        caption: [#hr[Technische Daten]],
    )
])
#pagebreak()