#import "../manual.typ": *

= #hr[Troubleshooting]
<trouble-shooting>

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

						[#tr[The LARUS Vario display starts up, but the satellite icon is red and the Vario indicators are frozen.]],
						[#tr[Connect to the LARUS CAN port using a crossed Rx/Tx patch cable instead of a standard 1:1 patch cable.]],
						[#tr[Please replace the patch cable and use the cable supplied with the product.]],

						[#tr[The LARUS Vario display starts up, but the satellite icon is red and the Vario indicators are frozen.]],
						[#tr[The LARUS Vario display has been connected to the wrong port (RS232).]],
						[#tr[Please connect the CAN ports.]], 

						[#tr[The satellite icon is constantly or frequently yellow; the altimeter and/or wind readings are implausible.]],
						[#tr[Poor GNSS reception]],
						[#tr[Ensure that the GNSS antenna is positioned facing upwards without any (metallic) shielding.]],

						[#tr[The Vario and/or wind readings are consistently or occasionally implausible.]],
						[#tr[The LARUS sensor unit is affected by magnetic interference.]],
						[#tr[Do not place the LARUS sensor unit near (moving) iron parts or magnets.]],

						[#tr[The Vario and/or wind readings are implausible.]],
						[#tr[The mounting position of the LARUS sensor unit has not been calibrated.]],
						[#tr[Perform the calibration (@sensorunit-calibration).]],

            table.hline(),
        ),
        caption: [#hr[Pin Configuration for CAN and RS232 RJ45]],
    )
])

#pagebreak()