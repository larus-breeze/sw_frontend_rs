// Created by create_polar_idx.py
#![allow(clippy::all)]

pub const TO_SORTED: &'static[u8] = &[
    0, // &'static str
    1, // 206 Hornet
    2, // 303 Mosquito
    3, // 304CZ
    4, // 401 Kestrel 17m
    5, // 604 Kestrel
    6, // AK-8
    9, // ASG-29 (15m)
    10, // ASG-29 (18m)
    11, // ASG-29E (15m)
    12, // ASG-29E (18m)
    13, // ASG-32
    14, // ASH-25
    15, // ASH-26
    16, // ASH-26E
    17, // ASK-13
    18, // ASK-18
    19, // ASK-21
    20, // ASK-23
    21, // ASW-12
    22, // ASW-15
    23, // ASW-17
    24, // ASW-19
    25, // ASW-20
    26, // ASW-20BL
    27, // ASW-22B
    28, // ASW-22BLE
    29, // ASW-24
    30, // ASW-27
    31, // ASW-28 (15m)
    32, // ASW-28 (18m)
    33, // Antares 18S
    34, // Antares 18T
    35, // Antares 20E
    36, // Apis (13m)
    37, // Apis 2 (15m)
    38, // Arcus
    39, // Blanik L13
    40, // Blanik L13-AC
    41, // Blanik L23
    42, // Carat
    43, // Cirrus (18m)
    44, // DG-100
    45, // DG-1000 (20m)
    46, // DG-200
    47, // DG-300
    48, // DG-400 (15m)
    49, // DG-400 (17m)
    50, // DG-500 (20m)
    51, // DG-600 (15m)
    52, // DG-800B (15m)
    53, // DG-800B (18m)
    54, // DG-800S (15m)
    55, // DG-800S (18m)
    56, // Delta USHPA-2
    57, // Delta USHPA-3
    58, // Delta USHPA-4
    59, // Dimona
    60, // Discus
    61, // Discus 2b
    62, // Discus 2c (18m)
    63, // Duo Discus
    64, // Duo Discus T
    65, // Duo Discus xT
    66, // EB 28
    67, // EB 28 Edition
    68, // G 102 Astir CS
    69, // G 103 Twin 2
    70, // G102 Club Astir
    71, // G102 Std Astir
    72, // G104 Speed Astir
    73, // Genesis II
    74, // Glasfluegel 304
    75, // H-301 Libelle
    76, // H201 Std Libelle
    77, // H205 Club Libelle
    78, // IS-28B2
    79, // IS-29D2 Lark
    80, // JS-1B (18m)
    81, // JS-1C (21m)
    82, // JS-3 (15m)
    83, // JS-3 (18m)
    84, // Janus (18m)
    85, // Janus C FG
    86, // Janus C RG
    87, // Ka 2b
    88, // Ka 4
    89, // Ka 6CR
    90, // Ka 6E
    91, // Ka 7
    92, // Ka 8
    93, // L 33 Solo
    94, // LAK-12
    95, // LAK-17 (15m)
    96, // LAK-17 (18m)
    97, // LAK-19 (15m)
    98, // LAK-19 (18m)
    99, // LAK17a (15m)
    100, // LAK17a (18m)
    101, // LS-10s (15m)
    102, // LS-10s (18m)
    103, // LS-1c
    104, // LS-1f
    105, // LS-3
    106, // LS-3 (17m)
    107, // LS-3 WL
    108, // LS-4
    109, // LS-5
    110, // LS-6 (15m)
    111, // LS-6 (18m)
    112, // LS-7wl
    113, // LS-8 (15m)
    114, // LS-8 (18m)
    115, // Mini Nimbus
    116, // Nimbus 2
    117, // Nimbus 3
    118, // Nimbus 3D
    119, // Nimbus 3DM
    120, // Nimbus 3T
    121, // Nimbus 4
    122, // Nimbus 4D
    123, // Nimbus 4DM
    124, // PIK-20B
    125, // PIK-20D
    126, // PIK-20E
    127, // PIK-30M
    128, // PW-5 Smyk
    129, // PW-6
    130, // Pegase 101A
    131, // Phoebus C
    132, // Pilatus B4 FG
    133, // R-26S Gobe
    134, // Russia AC-4
    135, // SF-27B
    136, // SGS 1-26E
    137, // SGS 1-34
    138, // SGS 1-35A
    139, // SGS 1-36 Sprite
    140, // SGS 2-33
    141, // SZD-30 Pirat
    142, // SZD-36 Cobra
    143, // SZD-42 Jantar II
    144, // SZD-48-2 Jantar
    145, // SZD-48-3 Jantar
    146, // SZD-50 Puchacz
    147, // SZD-51-1 Junior
    148, // SZD-54-2 17m
    149, // SZD-54-2 17m WL
    150, // SZD-54-2 20m WL
    151, // SZD-55-1 Promyk
    152, // SZD-9-1E Bocian
    153, // Silene E78
    154, // Skylark 4
    155, // Std Cirrus
    156, // Stemme S-10
    157, // Taurus
    158, // VSO-10 Gradient
    159, // VT-116 Orlik II
    160, // Ventus 2b 15m
    161, // Ventus 2c 18m
    162, // Ventus 2cT 18m
    163, // Ventus 2cx 18m
    164, // Ventus 2cxT 18m
    165, // Ventus a/b 16.6m
    166, // Ventus b (15m)
    167, // Ventus cM (17.6)
    168, // WA 26 P Squale
    169, // Zuni II
    8, // AS-33 18m
    7, // AS-33 15m
];

pub const TO_RAW: &'static[u8] = &[
    0, //&'static str
    1, //206 Hornet
    2, //303 Mosquito
    3, //304CZ
    4, //401 Kestrel 17m
    5, //604 Kestrel
    6, //AK-8
    169, //AS-33 15m
    168, //AS-33 18m
    7, //ASG-29 (15m)
    8, //ASG-29 (18m)
    9, //ASG-29E (15m)
    10, //ASG-29E (18m)
    11, //ASG-32
    12, //ASH-25
    13, //ASH-26
    14, //ASH-26E
    15, //ASK-13
    16, //ASK-18
    17, //ASK-21
    18, //ASK-23
    19, //ASW-12
    20, //ASW-15
    21, //ASW-17
    22, //ASW-19
    23, //ASW-20
    24, //ASW-20BL
    25, //ASW-22B
    26, //ASW-22BLE
    27, //ASW-24
    28, //ASW-27
    29, //ASW-28 (15m)
    30, //ASW-28 (18m)
    31, //Antares 18S
    32, //Antares 18T
    33, //Antares 20E
    34, //Apis (13m)
    35, //Apis 2 (15m)
    36, //Arcus
    37, //Blanik L13
    38, //Blanik L13-AC
    39, //Blanik L23
    40, //Carat
    41, //Cirrus (18m)
    42, //DG-100
    43, //DG-1000 (20m)
    44, //DG-200
    45, //DG-300
    46, //DG-400 (15m)
    47, //DG-400 (17m)
    48, //DG-500 (20m)
    49, //DG-600 (15m)
    50, //DG-800B (15m)
    51, //DG-800B (18m)
    52, //DG-800S (15m)
    53, //DG-800S (18m)
    54, //Delta USHPA-2
    55, //Delta USHPA-3
    56, //Delta USHPA-4
    57, //Dimona
    58, //Discus
    59, //Discus 2b
    60, //Discus 2c (18m)
    61, //Duo Discus
    62, //Duo Discus T
    63, //Duo Discus xT
    64, //EB 28
    65, //EB 28 Edition
    66, //G 102 Astir CS
    67, //G 103 Twin 2
    68, //G102 Club Astir
    69, //G102 Std Astir
    70, //G104 Speed Astir
    71, //Genesis II
    72, //Glasfluegel 304
    73, //H-301 Libelle
    74, //H201 Std Libelle
    75, //H205 Club Libelle
    76, //IS-28B2
    77, //IS-29D2 Lark
    78, //JS-1B (18m)
    79, //JS-1C (21m)
    80, //JS-3 (15m)
    81, //JS-3 (18m)
    82, //Janus (18m)
    83, //Janus C FG
    84, //Janus C RG
    85, //Ka 2b
    86, //Ka 4
    87, //Ka 6CR
    88, //Ka 6E
    89, //Ka 7
    90, //Ka 8
    91, //L 33 Solo
    92, //LAK-12
    93, //LAK-17 (15m)
    94, //LAK-17 (18m)
    95, //LAK-19 (15m)
    96, //LAK-19 (18m)
    97, //LAK17a (15m)
    98, //LAK17a (18m)
    99, //LS-10s (15m)
    100, //LS-10s (18m)
    101, //LS-1c
    102, //LS-1f
    103, //LS-3
    104, //LS-3 (17m)
    105, //LS-3 WL
    106, //LS-4
    107, //LS-5
    108, //LS-6 (15m)
    109, //LS-6 (18m)
    110, //LS-7wl
    111, //LS-8 (15m)
    112, //LS-8 (18m)
    113, //Mini Nimbus
    114, //Nimbus 2
    115, //Nimbus 3
    116, //Nimbus 3D
    117, //Nimbus 3DM
    118, //Nimbus 3T
    119, //Nimbus 4
    120, //Nimbus 4D
    121, //Nimbus 4DM
    122, //PIK-20B
    123, //PIK-20D
    124, //PIK-20E
    125, //PIK-30M
    126, //PW-5 Smyk
    127, //PW-6
    128, //Pegase 101A
    129, //Phoebus C
    130, //Pilatus B4 FG
    131, //R-26S Gobe
    132, //Russia AC-4
    133, //SF-27B
    134, //SGS 1-26E
    135, //SGS 1-34
    136, //SGS 1-35A
    137, //SGS 1-36 Sprite
    138, //SGS 2-33
    139, //SZD-30 Pirat
    140, //SZD-36 Cobra
    141, //SZD-42 Jantar II
    142, //SZD-48-2 Jantar
    143, //SZD-48-3 Jantar
    144, //SZD-50 Puchacz
    145, //SZD-51-1 Junior
    146, //SZD-54-2 17m
    147, //SZD-54-2 17m WL
    148, //SZD-54-2 20m WL
    149, //SZD-55-1 Promyk
    150, //SZD-9-1E Bocian
    151, //Silene E78
    152, //Skylark 4
    153, //Std Cirrus
    154, //Stemme S-10
    155, //Taurus
    156, //VSO-10 Gradient
    157, //VT-116 Orlik II
    158, //Ventus 2b 15m
    159, //Ventus 2c 18m
    160, //Ventus 2cT 18m
    161, //Ventus 2cx 18m
    162, //Ventus 2cxT 18m
    163, //Ventus a/b 16.6m
    164, //Ventus b (15m)
    165, //Ventus cM (17.6)
    166, //WA 26 P Squale
    167, //Zuni II
];
