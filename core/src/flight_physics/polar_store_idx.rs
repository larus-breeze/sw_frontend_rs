// Created by create_polar_idx.py
#![allow(clippy::all)]

pub const TO_SORTED: &'static[u8] = &[
    2, // 206 Hornet
    4, // 303 Mosquito
    5, // 304 CZ
    6, // 401 Kestrel 17m
    7, // 604 Kestrel
    8, // AK-8
    11, // ASG 29 15m
    12, // ASG 29 18m
    13, // ASG 29E 15m
    14, // ASG 29E 18m
    15, // ASG 32
    16, // ASH 25
    17, // ASH 26
    18, // ASH 26E
    21, // ASK 13
    22, // ASK 18
    23, // ASK 21
    24, // ASK 23
    25, // ASW 12
    26, // ASW 15
    27, // ASW 17
    28, // ASW 19
    29, // ASW 20
    30, // ASW 20BL
    31, // ASW 22B
    32, // ASW 22BLE
    33, // ASW 24
    34, // ASW 27
    35, // ASW 28 15m
    36, // ASW 28 18m
    37, // Antares 18S
    38, // Antares 18T
    39, // Antares 20E
    40, // Apis 13m
    41, // Apis 2 15m
    42, // Arcus
    43, // Blanik L13
    44, // Blanik L13-AC
    45, // Blanik L23
    46, // Carat
    47, // Cirrus 18m
    48, // DG-100
    49, // DG-1000 20m
    50, // DG-200
    51, // DG-300
    52, // DG-400 15m
    53, // DG-400 17m
    54, // DG-500 20m
    55, // DG-600 15m
    56, // DG-800B 15m
    57, // DG-800B 18m
    58, // DG-800S 15m
    59, // DG-800S 18m
    60, // Delta USHPA-2
    61, // Delta USHPA-3
    62, // Delta USHPA-4
    63, // Dimona
    64, // Discus
    65, // Discus 2b
    66, // Discus 2c 18m
    67, // Duo Discus
    68, // Duo Discus T
    69, // Duo Discus xT
    70, // EB 28
    71, // EB 28 Edition
    72, // G102 Astir CS
    75, // G103 Twin 2
    73, // G102 Club Astir
    74, // G102 Std Astir
    76, // G104 Speed Astir
    77, // Genesis II
    78, // Glasfluegel 304
    3, // 301 Libelle
    0, // 201 Std Libelle
    1, // 205 Club Libelle
    79, // IS-28B2
    80, // IS-29D2 Lark
    81, // JS-1B 18m
    82, // JS-1C 21m
    83, // JS-3 15m
    84, // JS-3 18m
    85, // Janus 18m
    86, // Janus C FG
    87, // Janus C RG
    88, // Ka 2b
    89, // Ka 4
    90, // Ka 6CR
    91, // Ka 6E
    92, // Ka 7
    93, // Ka 8
    94, // L 33 Solo
    95, // LAK-12
    96, // LAK-17 15m
    97, // LAK-17 18m
    98, // LAK-19 15m
    99, // LAK-19 18m
    100, // LAK17a 15m
    101, // LAK17a 18m
    102, // LS-10s 15m
    103, // LS-10s 18m
    104, // LS-1c
    105, // LS-1f
    106, // LS-3
    107, // LS-3 17m
    108, // LS-3 WL
    109, // LS-4
    110, // LS-5
    111, // LS-6 15m
    112, // LS-6 18m
    113, // LS-7wl
    114, // LS-8 15m
    115, // LS-8 18m
    116, // Mini Nimbus
    117, // Nimbus 2
    118, // Nimbus 3
    119, // Nimbus 3D
    120, // Nimbus 3DM
    121, // Nimbus 3T
    122, // Nimbus 4
    123, // Nimbus 4D
    124, // Nimbus 4DM
    125, // PIK-20B
    126, // PIK-20D
    127, // PIK-20E
    128, // PIK-30M
    129, // PW-5 Smyk
    130, // PW-6
    131, // Pegase 101A
    132, // Phoebus C
    133, // Pilatus B4 FG
    134, // R-26S Gobe
    135, // Russia AC-4
    136, // SF-27B
    137, // SGS 1-26E
    138, // SGS 1-34
    139, // SGS 1-35A
    140, // SGS 1-36 Sprite
    141, // SGS 2-33
    142, // SZD-30 Pirat
    143, // SZD-36 Cobra
    144, // SZD-42 Jantar II
    145, // SZD-48-2 Jantar
    146, // SZD-48-3 Jantar
    147, // SZD-50 Puchacz
    148, // SZD-51-1 Junior
    149, // SZD-54-2 17m
    150, // SZD-54-2 17m WL
    151, // SZD-54-2 20m WL
    152, // SZD-55-1 Promyk
    153, // SZD-9-1E Bocian
    154, // Silene E78
    155, // Skylark 4
    156, // Std Cirrus
    157, // Stemme S-10
    158, // Taurus
    159, // VSO-10 Gradient
    160, // VT-116 Orlik II
    161, // Ventus 2b 15m
    162, // Ventus 2c 18m
    163, // Ventus 2cT 18m
    164, // Ventus 2cx 18m
    165, // Ventus 2cxT 18m
    167, // Ventus a/b 16.6m
    168, // Ventus b 15m
    169, // Ventus cM 17.6m
    170, // WA 26 P Squale
    171, // Zuni II
    10, // AS 33 18m
    9, // AS 33 15m
    166, // Ventus 3 18m
    20, // ASH 31 Mi 21m
    19, // ASH 31 Mi 18m
];

pub const TO_RAW: &'static[u8] = &[
    73, //201 Std Libelle
    74, //205 Club Libelle
    0, //206 Hornet
    72, //301 Libelle
    1, //303 Mosquito
    2, //304 CZ
    3, //401 Kestrel 17m
    4, //604 Kestrel
    5, //AK-8
    168, //AS 33 15m
    167, //AS 33 18m
    6, //ASG 29 15m
    7, //ASG 29 18m
    8, //ASG 29E 15m
    9, //ASG 29E 18m
    10, //ASG 32
    11, //ASH 25
    12, //ASH 26
    13, //ASH 26E
    171, //ASH 31 Mi 18m
    170, //ASH 31 Mi 21m
    14, //ASK 13
    15, //ASK 18
    16, //ASK 21
    17, //ASK 23
    18, //ASW 12
    19, //ASW 15
    20, //ASW 17
    21, //ASW 19
    22, //ASW 20
    23, //ASW 20BL
    24, //ASW 22B
    25, //ASW 22BLE
    26, //ASW 24
    27, //ASW 27
    28, //ASW 28 15m
    29, //ASW 28 18m
    30, //Antares 18S
    31, //Antares 18T
    32, //Antares 20E
    33, //Apis 13m
    34, //Apis 2 15m
    35, //Arcus
    36, //Blanik L13
    37, //Blanik L13-AC
    38, //Blanik L23
    39, //Carat
    40, //Cirrus 18m
    41, //DG-100
    42, //DG-1000 20m
    43, //DG-200
    44, //DG-300
    45, //DG-400 15m
    46, //DG-400 17m
    47, //DG-500 20m
    48, //DG-600 15m
    49, //DG-800B 15m
    50, //DG-800B 18m
    51, //DG-800S 15m
    52, //DG-800S 18m
    53, //Delta USHPA-2
    54, //Delta USHPA-3
    55, //Delta USHPA-4
    56, //Dimona
    57, //Discus
    58, //Discus 2b
    59, //Discus 2c 18m
    60, //Duo Discus
    61, //Duo Discus T
    62, //Duo Discus xT
    63, //EB 28
    64, //EB 28 Edition
    65, //G102 Astir CS
    67, //G102 Club Astir
    68, //G102 Std Astir
    66, //G103 Twin 2
    69, //G104 Speed Astir
    70, //Genesis II
    71, //Glasfluegel 304
    75, //IS-28B2
    76, //IS-29D2 Lark
    77, //JS-1B 18m
    78, //JS-1C 21m
    79, //JS-3 15m
    80, //JS-3 18m
    81, //Janus 18m
    82, //Janus C FG
    83, //Janus C RG
    84, //Ka 2b
    85, //Ka 4
    86, //Ka 6CR
    87, //Ka 6E
    88, //Ka 7
    89, //Ka 8
    90, //L 33 Solo
    91, //LAK-12
    92, //LAK-17 15m
    93, //LAK-17 18m
    94, //LAK-19 15m
    95, //LAK-19 18m
    96, //LAK17a 15m
    97, //LAK17a 18m
    98, //LS-10s 15m
    99, //LS-10s 18m
    100, //LS-1c
    101, //LS-1f
    102, //LS-3
    103, //LS-3 17m
    104, //LS-3 WL
    105, //LS-4
    106, //LS-5
    107, //LS-6 15m
    108, //LS-6 18m
    109, //LS-7wl
    110, //LS-8 15m
    111, //LS-8 18m
    112, //Mini Nimbus
    113, //Nimbus 2
    114, //Nimbus 3
    115, //Nimbus 3D
    116, //Nimbus 3DM
    117, //Nimbus 3T
    118, //Nimbus 4
    119, //Nimbus 4D
    120, //Nimbus 4DM
    121, //PIK-20B
    122, //PIK-20D
    123, //PIK-20E
    124, //PIK-30M
    125, //PW-5 Smyk
    126, //PW-6
    127, //Pegase 101A
    128, //Phoebus C
    129, //Pilatus B4 FG
    130, //R-26S Gobe
    131, //Russia AC-4
    132, //SF-27B
    133, //SGS 1-26E
    134, //SGS 1-34
    135, //SGS 1-35A
    136, //SGS 1-36 Sprite
    137, //SGS 2-33
    138, //SZD-30 Pirat
    139, //SZD-36 Cobra
    140, //SZD-42 Jantar II
    141, //SZD-48-2 Jantar
    142, //SZD-48-3 Jantar
    143, //SZD-50 Puchacz
    144, //SZD-51-1 Junior
    145, //SZD-54-2 17m
    146, //SZD-54-2 17m WL
    147, //SZD-54-2 20m WL
    148, //SZD-55-1 Promyk
    149, //SZD-9-1E Bocian
    150, //Silene E78
    151, //Skylark 4
    152, //Std Cirrus
    153, //Stemme S-10
    154, //Taurus
    155, //VSO-10 Gradient
    156, //VT-116 Orlik II
    157, //Ventus 2b 15m
    158, //Ventus 2c 18m
    159, //Ventus 2cT 18m
    160, //Ventus 2cx 18m
    161, //Ventus 2cxT 18m
    169, //Ventus 3 18m
    162, //Ventus a/b 16.6m
    163, //Ventus b 15m
    164, //Ventus cM 17.6m
    165, //WA 26 P Squale
    166, //Zuni II
];
