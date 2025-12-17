#import "../manual.typ": *

= #hr[Troubleshooting]

#h(5mm)
#align(left, block[
    #figure(
        table(
            columns: (auto, auto, auto),
            align: (left, left, left),
            table.hline(),
            table.header(
                [#hr[*Problem*]], [#hr[*Possible Causes*]], [#hr[*Solutions*]],
            ),
            table.hline(),

						[#tr[The LARUS Vario Display starts up, but the satellite icon is red and the vario pointers are fixed.]],
						[#tr[Connection to the LARUS CAN port using a crossed Rx/Tx patch cable instead of a standard 1:1 patch cable.]],
						[#tr[Please replace the patch cable and use the cable included in the delivery.]],

						[#tr[The LARUS Vario Display starts up, but the satellite icon is red and the vario pointers are fixed.]],
						[#tr[The LARUS Vario display has been connected to the wrong connector (RS232).]],
						[#tr[Please connect the CAN ports.]], 

						[#tr[The satellite pictogram is constantly or frequently yellow, and the vario and/or wind values are not plausible.]],
						[#tr[Poor GNSS reception]],
						[#tr[Ensure that the GNSS antenna is positioned upwards without any (metallic) shielding.]],

						[#tr[Variable and/or wind values are permanently or temporarily implausible.]],
						[#tr[The LARUS sensor unit is disrupted by magnetic influences.]],
						[#tr[Do not place the LARUS sensor unit near (moving) iron parts or magnets.]],

						[#tr[Variable and/or wind values are not plausible.]],
						[#tr[The installation position of the LARUS sensor unit has not been calibrated.]],
						[#tr[Perform the calibration} (\nameref{sensorunit-calibration}).]],

            table.hline(),
        ),
        caption: [#hr[CAN and RS232 RJ45 Pin Assignment]],
    )
])

#pagebreak()