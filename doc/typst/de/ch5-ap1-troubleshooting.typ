#import "../manual.typ": *

= #hr[Fehlerbehebung]

#h(5mm)
#align(left, block[
    #figure(
        table(
            columns: (auto, auto, auto),
            align: (left, left, left),
            table.hline(),
            table.header(
                [#hr[*Problem*]], [#hr[*Mögliche Ursachen*]], [#hr[*Lösungen*]],
            ),
            table.hline(),

						[#tr[Das LARUS Vario Display startet, jedoch ist das Satellitenpiktogramm rot und die Vario-Zeiger sind fixiert.]],
						[#tr[Anschluss an den LARUS-CAN-Port mit einem gekreuzten Rx/Tx-Patchkabel anstelle eines standardmäßigen 1:1-Patchkabels.]],
						[#tr[Bitte ersetzen Sie das Patchkabel und verwenden Sie das im Lieferumfang enthaltene Kabel.]],

						[#tr[Das LARUS Vario Display startet, jedoch ist das Satellitenpiktogramm rot und die Vario-Zeiger sind fixiert.]],
						[#tr[Das LARUS Vario Display wurde am falschen Stecker angeschlossen (RS232).]],
						[#tr[Bitte verbinden Sie die CAN-Anschlüsse.]], 

						[#tr[Das Satellitenpiktogramm ist dauernd oder öfters gelb, die Vario- und/oder Windwerte sind nicht plausibel.]],
						[#tr[Schlechter GNSS Empfang]],
						[#tr[Sorgen Sie dafür, dass die GNSS Antenne ohne (metallische) Abschirmung nach oben plaziert ist.]],

						[#tr[Vario- und/oder Windwerte sind dauernd oder zeitweise nicht plausibel.]],
						[#tr[Die LARUS Sensoreinheit wird durch magnetische Einflüsse gestört.]],
						[#tr[Platzieren Sie die LARUS Sensoreinheit nicht in der Nähe von (beweglichen) Eisenteilen oder Magneten.]],

						[#tr[Vario- und/oder Windwerte sind nicht plausibel.]],
						[#tr[Die Einbaulage der LARUS Sensoreinheit wurde nicht kalibriert.]],
						[#tr[Führen Sie die Kalibrierung durch (@sensorunit-calibration).]],

            table.hline(),
        ),
        caption: [#hr[Pinbelegung CAN und RS232 RJ45]],
    )
])

#pagebreak()