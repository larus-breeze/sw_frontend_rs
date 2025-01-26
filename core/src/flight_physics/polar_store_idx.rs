// Created by create_polar_idx.py

pub const TO_SORTED: &'static[u8] = &[
    0, // 206 Hornet
    1, // 303 Mosquito
    2, // 304CZ
    3, // 401 Kestrel 17m
    4, // 604 Kestrel
    5, // AK-8
    8, // ASG-29 (15m)
    9, // ASG-29 (18m)
    10, // ASG-29E (15m)
    11, // ASG-29E (18m)
    12, // ASG-32
    13, // ASH-25
    14, // ASH-26
    15, // ASH-26E
    16, // ASK-13
    17, // ASK-18
    18, // ASK-21
    19, // ASK-23
    20, // ASW-12
    21, // ASW-15
    22, // ASW-17
    23, // ASW-19
    24, // ASW-20
    25, // ASW-20BL
    26, // ASW-22B
    27, // ASW-22BLE
    28, // ASW-24
    29, // ASW-27
    30, // ASW-28 (15m)
    31, // ASW-28 (18m)
    32, // Antares 18S
    33, // Antares 18T
    34, // Antares 20E
    35, // Apis (13m)
    36, // Apis 2 (15m)
    37, // Arcus
    38, // Blanik L13
    39, // Blanik L13-AC
    40, // Blanik L23
    41, // Carat
    42, // Cirrus (18m)
    43, // DG-100
    44, // DG-1000 (20m)
    45, // DG-200
    46, // DG-300
    47, // DG-400 (15m)
    48, // DG-400 (17m)
    49, // DG-500 (20m)
    50, // DG-600 (15m)
    51, // DG-800B (15m)
    52, // DG-800B (18m)
    53, // DG-800S (15m)
    54, // DG-800S (18m)
    55, // Delta USHPA-2
    56, // Delta USHPA-3
    57, // Delta USHPA-4
    58, // Dimona
    59, // Discus
    60, // Discus 2b
    61, // Discus 2c (18m)
    62, // Duo Discus
    63, // Duo Discus T
    64, // Duo Discus xT
    65, // EB 28
    66, // EB 28 Edition
    67, // G 102 Astir CS
    68, // G 103 Twin 2
    69, // G102 Club Astir
    70, // G102 Std Astir
    71, // G104 Speed Astir
    72, // Genesis II
    73, // Glasfluegel 304
    74, // H-301 Libelle
    75, // H201 Std Libelle
    76, // H205 Club Libelle
    77, // IS-28B2
    78, // IS-29D2 Lark
    79, // JS-1B (18m)
    80, // JS-1C (21m)
    81, // JS-3 (15m)
    82, // JS-3 (18m)
    83, // Janus (18m)
    84, // Janus C FG
    85, // Janus C RG
    86, // Ka 2b
    87, // Ka 4
    88, // Ka 6CR
    89, // Ka 6E
    90, // Ka 7
    91, // Ka 8
    92, // L 33 Solo
    93, // LAK-12
    94, // LAK-17 (15m)
    95, // LAK-17 (18m)
    96, // LAK-19 (15m)
    97, // LAK-19 (18m)
    98, // LAK17a (15m)
    99, // LAK17a (18m)
    100, // LS-10s (15m)
    101, // LS-10s (18m)
    102, // LS-1c
    103, // LS-1f
    104, // LS-3
    105, // LS-3 (17m)
    106, // LS-3 WL
    107, // LS-4
    108, // LS-5
    109, // LS-6 (15m)
    110, // LS-6 (18m)
    111, // LS-7wl
    112, // LS-8 (15m)
    113, // LS-8 (18m)
    114, // Mini Nimbus
    115, // Nimbus 2
    116, // Nimbus 3
    117, // Nimbus 3D
    118, // Nimbus 3DM
    119, // Nimbus 3T
    120, // Nimbus 4
    121, // Nimbus 4D
    122, // Nimbus 4DM
    123, // PIK-20B
    124, // PIK-20D
    125, // PIK-20E
    126, // PIK-30M
    127, // PW-5 Smyk
    128, // PW-6
    129, // Pegase 101A
    130, // Phoebus C
    131, // Pilatus B4 FG
    132, // R-26S Gobe
    133, // Russia AC-4
    134, // SF-27B
    135, // SGS 1-26E
    136, // SGS 1-34
    137, // SGS 1-35A
    138, // SGS 1-36 Sprite
    139, // SGS 2-33
    140, // SZD-30 Pirat
    141, // SZD-36 Cobra
    142, // SZD-42 Jantar II
    143, // SZD-48-2 Jantar
    144, // SZD-48-3 Jantar
    145, // SZD-50 Puchacz
    146, // SZD-51-1 Junior
    147, // SZD-54-2 17m
    148, // SZD-54-2 17m WL
    149, // SZD-54-2 20m WL
    150, // SZD-55-1 Promyk
    151, // SZD-9-1E Bocian
    152, // Silene E78
    153, // Skylark 4
    154, // Std Cirrus
    155, // Stemme S-10
    156, // Taurus
    157, // VSO-10 Gradient
    158, // VT-116 Orlik II
    159, // Ventus 2b 15m
    160, // Ventus 2c 18m
    161, // Ventus 2cT 18m
    162, // Ventus 2cx 18m
    163, // Ventus 2cxT 18m
    164, // Ventus a/b 16.6m
    165, // Ventus b (15m)
    166, // Ventus cM (17.6)
    167, // WA 26 P Squale
    168, // Zuni II
    7, // AS-33 18m
    6, // AS-33 15m
];

pub const TO_RAW: &'static[u8] = &[
    0, //206 Hornet
    1, //303 Mosquito
    2, //304CZ
    3, //401 Kestrel 17m
    4, //604 Kestrel
    5, //AK-8
    168, //AS-33 15m
    167, //AS-33 18m
    6, //ASG-29 (15m)
    7, //ASG-29 (18m)
    8, //ASG-29E (15m)
    9, //ASG-29E (18m)
    10, //ASG-32
    11, //ASH-25
    12, //ASH-26
    13, //ASH-26E
    14, //ASK-13
    15, //ASK-18
    16, //ASK-21
    17, //ASK-23
    18, //ASW-12
    19, //ASW-15
    20, //ASW-17
    21, //ASW-19
    22, //ASW-20
    23, //ASW-20BL
    24, //ASW-22B
    25, //ASW-22BLE
    26, //ASW-24
    27, //ASW-27
    28, //ASW-28 (15m)
    29, //ASW-28 (18m)
    30, //Antares 18S
    31, //Antares 18T
    32, //Antares 20E
    33, //Apis (13m)
    34, //Apis 2 (15m)
    35, //Arcus
    36, //Blanik L13
    37, //Blanik L13-AC
    38, //Blanik L23
    39, //Carat
    40, //Cirrus (18m)
    41, //DG-100
    42, //DG-1000 (20m)
    43, //DG-200
    44, //DG-300
    45, //DG-400 (15m)
    46, //DG-400 (17m)
    47, //DG-500 (20m)
    48, //DG-600 (15m)
    49, //DG-800B (15m)
    50, //DG-800B (18m)
    51, //DG-800S (15m)
    52, //DG-800S (18m)
    53, //Delta USHPA-2
    54, //Delta USHPA-3
    55, //Delta USHPA-4
    56, //Dimona
    57, //Discus
    58, //Discus 2b
    59, //Discus 2c (18m)
    60, //Duo Discus
    61, //Duo Discus T
    62, //Duo Discus xT
    63, //EB 28
    64, //EB 28 Edition
    65, //G 102 Astir CS
    66, //G 103 Twin 2
    67, //G102 Club Astir
    68, //G102 Std Astir
    69, //G104 Speed Astir
    70, //Genesis II
    71, //Glasfluegel 304
    72, //H-301 Libelle
    73, //H201 Std Libelle
    74, //H205 Club Libelle
    75, //IS-28B2
    76, //IS-29D2 Lark
    77, //JS-1B (18m)
    78, //JS-1C (21m)
    79, //JS-3 (15m)
    80, //JS-3 (18m)
    81, //Janus (18m)
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
    92, //LAK-17 (15m)
    93, //LAK-17 (18m)
    94, //LAK-19 (15m)
    95, //LAK-19 (18m)
    96, //LAK17a (15m)
    97, //LAK17a (18m)
    98, //LS-10s (15m)
    99, //LS-10s (18m)
    100, //LS-1c
    101, //LS-1f
    102, //LS-3
    103, //LS-3 (17m)
    104, //LS-3 WL
    105, //LS-4
    106, //LS-5
    107, //LS-6 (15m)
    108, //LS-6 (18m)
    109, //LS-7wl
    110, //LS-8 (15m)
    111, //LS-8 (18m)
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
    162, //Ventus a/b 16.6m
    163, //Ventus b (15m)
    164, //Ventus cM (17.6)
    165, //WA 26 P Squale
    166, //Zuni II
];
