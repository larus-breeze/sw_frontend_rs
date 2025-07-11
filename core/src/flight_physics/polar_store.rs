//! This file contains all polars in any order. Please note:
//!
//! Do not delete any polars, as these could have been selected by a user
//!
//! When adding, append the new polars to the end and then start the script
//! “assets/crate_polar_idx.py”. This script creates an index that enables alphabetical
//! selection in the menu.

/// Structure with the basic data of a sailplane
///
/// The contents are kept as natural Rust data types and not in the physical sizes. This data
/// structure is used as the basis for the polar. The contents correspond to the polar store
/// in XCSoar
#[derive(Copy, Clone)]
pub struct BasicGliderData {
    pub name: &'static str,
    pub wing_area: f32,        // m²
    pub max_speed: f32,        // km/h
    pub empty_mass: f32,       // km/h
    pub max_ballast: f32,      // kg
    pub reference_weight: f32, // kg
    pub handicap: u16,
    pub polar_values: [[f32; 2]; 3], // (km/h, m/s) * 3
}

impl Default for BasicGliderData {
    fn default() -> Self {
        BasicGliderData {
            name: "",
            wing_area: 0.0,
            max_speed: 0.0,
            empty_mass: 0.0,
            max_ballast: 0.0,
            reference_weight: 0.0,
            handicap: 0,
            polar_values: [[0.0, 0.0], [0.0, 0.0], [0.0, 0.0]],
        }
    }
}

use super::polar_store_idx::{TO_RAW, TO_SORTED};

pub fn to_sorted_idx(raw_idx: usize) -> usize {
    TO_SORTED[raw_idx] as usize
}

pub fn to_raw_idx(sorted_idx: usize) -> usize {
    TO_RAW[sorted_idx] as usize
}

pub fn from_raw_idx(raw_idx: usize) -> &'static BasicGliderData {
    &POLARS[raw_idx]
}

pub fn size() -> usize {
    // We use generated TO_RAW to be save against forgotten script run
    TO_RAW.len()
}

#[allow(unused)]
pub const POLARS: &[BasicGliderData] = &[
    BasicGliderData {
        // No 0,  imported from XCSoar
        name: "206 Hornet",
        wing_area: 9.80,
        max_speed: 250.0,
        empty_mass: 227.0,
        max_ballast: 100.0,
        reference_weight: 318.0,
        handicap: 100,
        polar_values: [[80.0, -0.606], [120.0, -0.990], [160.0, -1.918]],
    },
    BasicGliderData {
        // No 1,  imported from XCSoar
        name: "303 Mosquito",
        wing_area: 9.85,
        max_speed: 250.0,
        empty_mass: 242.0,
        max_ballast: 0.0,
        reference_weight: 450.0,
        handicap: 107,
        polar_values: [[100.0, -0.680], [120.0, -0.920], [150.0, -1.450]],
    },
    BasicGliderData {
        // No 2,  imported from XCSoar
        name: "304CZ",
        wing_area: 0.00,
        max_speed: 250.0,
        empty_mass: 235.0,
        max_ballast: 115.0,
        reference_weight: 310.0,
        handicap: 110,
        polar_values: [[115.0, -0.860], [174.0, -1.760], [212.7, -3.400]],
    },
    BasicGliderData {
        // No 3,  imported from XCSoar
        name: "401 Kestrel 17m",
        wing_area: 11.58,
        max_speed: 250.0,
        empty_mass: 260.0,
        max_ballast: 33.0,
        reference_weight: 367.0,
        handicap: 110,
        polar_values: [[95.0, -0.620], [110.0, -0.760], [175.0, -2.010]],
    },
    BasicGliderData {
        // No 4,  imported from XCSoar
        name: "604 Kestrel",
        wing_area: 16.26,
        max_speed: 250.0,
        empty_mass: 455.0,
        max_ballast: 100.0,
        reference_weight: 570.0,
        handicap: 114,
        polar_values: [[113.0, -0.720], [150.6, -1.420], [207.1, -4.100]],
    },
    BasicGliderData {
        // No 5,  imported from XCSoar
        name: "AK-8",
        wing_area: 9.75,
        max_speed: 250.0,
        empty_mass: 233.0,
        max_ballast: 100.0,
        reference_weight: 362.0,
        handicap: 107,
        polar_values: [[84.1, -0.652], [130.0, -0.947], [170.0, -1.838]],
    },
    BasicGliderData {
        // No 6,  imported from XCSoar
        name: "ASG-29 (15m)",
        wing_area: 9.20,
        max_speed: 250.0,
        empty_mass: 270.0,
        max_ballast: 165.0,
        reference_weight: 362.0,
        handicap: 114,
        polar_values: [[108.8, -0.635], [156.4, -1.182], [211.1, -2.540]],
    },
    BasicGliderData {
        // No 7,  imported from XCSoar
        name: "ASG-29 (18m)",
        wing_area: 10.50,
        max_speed: 250.0,
        empty_mass: 280.0,
        max_ballast: 225.0,
        reference_weight: 355.0,
        handicap: 121,
        polar_values: [[85.0, -0.470], [90.0, -0.480], [185.0, -2.000]],
    },
    BasicGliderData {
        // No 8,  imported from XCSoar
        name: "ASG-29E (15m)",
        wing_area: 9.20,
        max_speed: 250.0,
        empty_mass: 315.0,
        max_ballast: 200.0,
        reference_weight: 350.0,
        handicap: 114,
        polar_values: [[100.0, -0.640], [120.0, -0.750], [150.0, -1.130]],
    },
    BasicGliderData {
        // No 9,  imported from XCSoar
        name: "ASG-29E (18m)",
        wing_area: 10.50,
        max_speed: 250.0,
        empty_mass: 325.0,
        max_ballast: 200.0,
        reference_weight: 400.0,
        handicap: 121,
        polar_values: [[90.0, -0.499], [95.5, -0.510], [196.4, -2.120]],
    },
    BasicGliderData {
        // No 10,  self added
        name: "ASG-32",
        wing_area: 15.70,
        max_speed: 250.0,
        empty_mass: 650.0,
        max_ballast: 125.0,
        reference_weight: 807.0,
        handicap: 120,
        polar_values: [[100.0, -0.582], [126.0, -0.648], [185.0, -1.450]],
    },
    BasicGliderData {
        // No 11,  imported from XCSoar
        name: "ASH-25",
        wing_area: 16.31,
        max_speed: 250.0,
        empty_mass: 478.0,
        max_ballast: 121.0,
        reference_weight: 750.0,
        handicap: 122,
        polar_values: [[130.0, -0.780], [170.0, -1.400], [219.9, -2.600]],
    },
    BasicGliderData {
        // No 12,  imported from XCSoar
        name: "ASH-26",
        wing_area: 11.70,
        max_speed: 250.0,
        empty_mass: 325.0,
        max_ballast: 185.0,
        reference_weight: 340.0,
        handicap: 119,
        polar_values: [[100.0, -0.560], [120.0, -0.740], [150.0, -1.160]],
    },
    BasicGliderData {
        // No 13,  imported from XCSoar
        name: "ASH-26E",
        wing_area: 11.70,
        max_speed: 250.0,
        empty_mass: 360.0,
        max_ballast: 90.0,
        reference_weight: 435.0,
        handicap: 119,
        polar_values: [[90.0, -0.510], [96.0, -0.530], [185.0, -2.000]],
    },
    BasicGliderData {
        // No 14,  imported from XCSoar
        name: "ASK-13",
        wing_area: 17.50,
        max_speed: 250.0,
        empty_mass: 296.0,
        max_ballast: 0.0,
        reference_weight: 456.0,
        handicap: 79,
        polar_values: [[85.0, -0.840], [120.0, -1.500], [150.0, -2.800]],
    },
    BasicGliderData {
        // No 15,  imported from XCSoar
        name: "ASK-18",
        wing_area: 12.99,
        max_speed: 250.0,
        empty_mass: 215.0,
        max_ballast: 0.0,
        reference_weight: 310.0,
        handicap: 88,
        polar_values: [[75.0, -0.613], [138.0, -1.773], [200.0, -4.234]],
    },
    BasicGliderData {
        // No 16,  imported from XCSoar
        name: "ASK-21",
        wing_area: 17.95,
        max_speed: 250.0,
        empty_mass: 360.0,
        max_ballast: 0.0,
        reference_weight: 468.0,
        handicap: 92,
        polar_values: [[74.1, -0.670], [101.9, -0.900], [166.7, -2.680]],
    },
    BasicGliderData {
        // No 17,  imported from XCSoar
        name: "ASK-23",
        wing_area: 12.90,
        max_speed: 250.0,
        empty_mass: 240.0,
        max_ballast: 0.0,
        reference_weight: 330.0,
        handicap: 92,
        polar_values: [[100.0, -0.850], [120.0, -1.190], [150.0, -2.020]],
    },
    BasicGliderData {
        // No 18,  imported from XCSoar
        name: "ASW-12",
        wing_area: 13.00,
        max_speed: 250.0,
        empty_mass: 324.0,
        max_ballast: 189.0,
        reference_weight: 394.0,
        handicap: 110,
        polar_values: [[95.0, -0.570], [148.0, -1.480], [183.1, -2.600]],
    },
    BasicGliderData {
        // No 19,  imported from XCSoar
        name: "ASW-15",
        wing_area: 11.00,
        max_speed: 250.0,
        empty_mass: 210.0,
        max_ballast: 91.0,
        reference_weight: 349.0,
        handicap: 97,
        polar_values: [[97.6, -0.770], [156.1, -1.900], [195.2, -3.400]],
    },
    BasicGliderData {
        // No 20,  imported from XCSoar
        name: "ASW-17",
        wing_area: 14.84,
        max_speed: 250.0,
        empty_mass: 405.0,
        max_ballast: 151.0,
        reference_weight: 522.0,
        handicap: 115,
        polar_values: [[114.5, -0.700], [169.1, -1.680], [206.5, -2.900]],
    },
    BasicGliderData {
        // No 21,  imported from XCSoar
        name: "ASW-19",
        wing_area: 11.00,
        max_speed: 250.0,
        empty_mass: 240.0,
        max_ballast: 125.0,
        reference_weight: 363.0,
        handicap: 100,
        polar_values: [[97.5, -0.740], [156.0, -1.640], [195.0, -3.100]],
    },
    BasicGliderData {
        // No 22,  imported from XCSoar
        name: "ASW-20",
        wing_area: 10.50,
        max_speed: 250.0,
        empty_mass: 255.0,
        max_ballast: 159.0,
        reference_weight: 377.0,
        handicap: 108,
        polar_values: [[116.2, -0.770], [174.3, -1.890], [213.0, -3.300]],
    },
    BasicGliderData {
        // No 23,  imported from XCSoar
        name: "ASW-20BL",
        wing_area: 10.49,
        max_speed: 250.0,
        empty_mass: 275.0,
        max_ballast: 126.0,
        reference_weight: 400.0,
        handicap: 112,
        polar_values: [[95.0, -0.628], [148.0, -1.338], [200.0, -2.774]],
    },
    BasicGliderData {
        // No 24,  imported from XCSoar
        name: "ASW-22B",
        wing_area: 16.31,
        max_speed: 250.0,
        empty_mass: 270.0,
        max_ballast: 303.0,
        reference_weight: 597.0,
        handicap: 123,
        polar_values: [[80.0, -0.402], [120.0, -0.660], [160.0, -1.354]],
    },
    BasicGliderData {
        // No 25,  imported from XCSoar
        name: "ASW-22BLE",
        wing_area: 16.70,
        max_speed: 250.0,
        empty_mass: 275.0,
        max_ballast: 285.0,
        reference_weight: 465.0,
        handicap: 124,
        polar_values: [[100.0, -0.470], [120.0, -0.630], [150.0, -1.040]],
    },
    BasicGliderData {
        // No 26,  imported from XCSoar
        name: "ASW-24",
        wing_area: 10.00,
        max_speed: 250.0,
        empty_mass: 230.0,
        max_ballast: 159.0,
        reference_weight: 350.0,
        handicap: 107,
        polar_values: [[108.8, -0.730], [142.2, -1.210], [167.4, -1.800]],
    },
    BasicGliderData {
        // No 27,  imported from XCSoar
        name: "ASW-27",
        wing_area: 9.00,
        max_speed: 250.0,
        empty_mass: 235.0,
        max_ballast: 165.0,
        reference_weight: 365.0,
        handicap: 114,
        polar_values: [[88.8, -0.594], [130.0, -0.851], [170.0, -1.610]],
    },
    BasicGliderData {
        // No 28,  imported from XCSoar
        name: "ASW-28 (15m)",
        wing_area: 10.50,
        max_speed: 250.0,
        empty_mass: 235.0,
        max_ballast: 200.0,
        reference_weight: 310.0,
        handicap: 108,
        polar_values: [[92.6, -0.571], [120.4, -0.875], [148.2, -1.394]],
    },
    BasicGliderData {
        // No 29,  imported from XCSoar
        name: "ASW-28 (18m)",
        wing_area: 10.50,
        max_speed: 250.0,
        empty_mass: 270.0,
        max_ballast: 190.0,
        reference_weight: 345.0,
        handicap: 114,
        polar_values: [[65.0, -0.470], [107.0, -0.670], [165.0, -2.000]],
    },
    BasicGliderData {
        // No 30,  imported from XCSoar
        name: "Antares 18S",
        wing_area: 10.97,
        max_speed: 250.0,
        empty_mass: 295.0,
        max_ballast: 250.0,
        reference_weight: 350.0,
        handicap: 120,
        polar_values: [[100.0, -0.540], [120.0, -0.630], [150.0, -1.070]],
    },
    BasicGliderData {
        // No 31,  imported from XCSoar
        name: "Antares 18T",
        wing_area: 10.97,
        max_speed: 250.0,
        empty_mass: 345.0,
        max_ballast: 205.0,
        reference_weight: 395.0,
        handicap: 120,
        polar_values: [[100.0, -0.540], [120.0, -0.690], [150.0, -1.110]],
    },
    BasicGliderData {
        // No 32,  imported from XCSoar
        name: "Antares 20E",
        wing_area: 12.60,
        max_speed: 250.0,
        empty_mass: 475.0,
        max_ballast: 130.0,
        reference_weight: 530.0,
        handicap: 123,
        polar_values: [[100.0, -0.520], [120.0, -0.610], [150.0, -0.910]],
    },
    BasicGliderData {
        // No 33,  imported from XCSoar
        name: "Apis (13m)",
        wing_area: 10.36,
        max_speed: 250.0,
        empty_mass: 137.0,
        max_ballast: 45.0,
        reference_weight: 200.0,
        handicap: 93,
        polar_values: [[100.0, -0.740], [120.0, -1.010], [150.0, -1.660]],
    },
    BasicGliderData {
        // No 34,  imported from XCSoar
        name: "Apis 2 (15m)",
        wing_area: 12.40,
        max_speed: 250.0,
        empty_mass: 215.0,
        max_ballast: 0.0,
        reference_weight: 310.0,
        handicap: 98,
        polar_values: [[80.0, -0.600], [100.0, -0.750], [140.0, -1.450]],
    },
    BasicGliderData {
        // No 35,  imported from XCSoar
        name: "Arcus",
        wing_area: 15.59,
        max_speed: 250.0,
        empty_mass: 430.0,
        max_ballast: 185.0,
        reference_weight: 700.0,
        handicap: 120,
        polar_values: [[110.0, -0.640], [140.0, -0.880], [180.0, -1.470]],
    },
    BasicGliderData {
        // No 36,  imported from XCSoar
        name: "Blanik L13",
        wing_area: 19.10,
        max_speed: 250.0,
        empty_mass: 292.0,
        max_ballast: 0.0,
        reference_weight: 472.0,
        handicap: 78,
        polar_values: [[85.0, -0.840], [143.0, -3.320], [200.0, -9.610]],
    },
    BasicGliderData {
        // No 37,  imported from XCSoar
        name: "Blanik L13-AC",
        wing_area: 17.44,
        max_speed: 250.0,
        empty_mass: 306.0,
        max_ballast: 0.0,
        reference_weight: 500.0,
        handicap: 78,
        polar_values: [[70.0, -0.850], [110.0, -1.250], [160.0, -3.200]],
    },
    BasicGliderData {
        // No 38,  imported from XCSoar
        name: "Blanik L23",
        wing_area: 19.10,
        max_speed: 250.0,
        empty_mass: 310.0,
        max_ballast: 0.0,
        reference_weight: 510.0,
        handicap: 80,
        polar_values: [[95.0, -0.940], [148.0, -2.600], [200.0, -6.370]],
    },
    BasicGliderData {
        // No 39,  imported from XCSoar
        name: "Carat",
        wing_area: 10.58,
        max_speed: 250.0,
        empty_mass: 341.0,
        max_ballast: 0.0,
        reference_weight: 470.0,
        handicap: 93,
        polar_values: [[100.0, -0.830], [120.0, -1.040], [150.0, -1.690]],
    },
    BasicGliderData {
        // No 40,  imported from XCSoar
        name: "Cirrus (18m)",
        wing_area: 12.60,
        max_speed: 250.0,
        empty_mass: 260.0,
        max_ballast: 100.0,
        reference_weight: 330.0,
        handicap: 102,
        polar_values: [[100.0, -0.740], [120.0, -1.060], [150.0, -1.880]],
    },
    BasicGliderData {
        // No 41,  imported from XCSoar
        name: "DG-100",
        wing_area: 11.00,
        max_speed: 250.0,
        empty_mass: 230.0,
        max_ballast: 100.0,
        reference_weight: 300.0,
        handicap: 100,
        polar_values: [[100.0, -0.730], [120.0, -1.000], [150.0, -1.700]],
    },
    BasicGliderData {
        // No 42,  imported from XCSoar
        name: "DG-1000 (20m)",
        wing_area: 17.51,
        max_speed: 250.0,
        empty_mass: 415.0,
        max_ballast: 160.0,
        reference_weight: 613.0,
        handicap: 111,
        polar_values: [[106.0, -0.620], [153.0, -1.530], [200.0, -3.200]],
    },
    BasicGliderData {
        // No 43,  imported from XCSoar
        name: "DG-200",
        wing_area: 10.00,
        max_speed: 250.0,
        empty_mass: 230.0,
        max_ballast: 120.0,
        reference_weight: 300.0,
        handicap: 107,
        polar_values: [[100.0, -0.680], [120.0, -0.860], [150.0, -1.300]],
    },
    BasicGliderData {
        // No 44,  imported from XCSoar
        name: "DG-300",
        wing_area: 10.27,
        max_speed: 250.0,
        empty_mass: 235.0,
        max_ballast: 190.0,
        reference_weight: 310.0,
        handicap: 104,
        polar_values: [[95.0, -0.660], [140.0, -1.280], [160.0, -1.700]],
    },
    BasicGliderData {
        // No 45,  imported from XCSoar
        name: "DG-400 (15m)",
        wing_area: 10.00,
        max_speed: 250.0,
        empty_mass: 306.0,
        max_ballast: 90.0,
        reference_weight: 440.0,
        handicap: 107,
        polar_values: [[115.0, -0.760], [160.5, -1.220], [210.2, -2.300]],
    },
    BasicGliderData {
        // No 46,  imported from XCSoar
        name: "DG-400 (17m)",
        wing_area: 10.57,
        max_speed: 250.0,
        empty_mass: 310.0,
        max_ballast: 90.0,
        reference_weight: 444.0,
        handicap: 109,
        polar_values: [[118.3, -0.680], [163.8, -1.150], [198.3, -1.800]],
    },
    BasicGliderData {
        // No 47,  imported from XCSoar
        name: "DG-500 (20m)",
        wing_area: 18.29,
        max_speed: 250.0,
        empty_mass: 390.0,
        max_ballast: 100.0,
        reference_weight: 659.0,
        handicap: 104,
        polar_values: [[115.4, -0.710], [152.0, -1.280], [190.0, -2.300]],
    },
    BasicGliderData {
        // No 48,  imported from XCSoar
        name: "DG-600 (15m)",
        wing_area: 10.95,
        max_speed: 250.0,
        empty_mass: 255.0,
        max_ballast: 180.0,
        reference_weight: 327.0,
        handicap: 110,
        polar_values: [[100.0, -0.600], [120.0, -0.760], [150.0, -1.190]],
    },
    BasicGliderData {
        // No 49,  imported from XCSoar
        name: "DG-800B (15m)",
        wing_area: 10.68,
        max_speed: 250.0,
        empty_mass: 340.0,
        max_ballast: 100.0,
        reference_weight: 468.0,
        handicap: 113,
        polar_values: [[103.6, -0.653], [130.0, -0.891], [170.0, -1.481]],
    },
    BasicGliderData {
        // No 50,  imported from XCSoar
        name: "DG-800B (18m)",
        wing_area: 11.81,
        max_speed: 250.0,
        empty_mass: 344.0,
        max_ballast: 100.0,
        reference_weight: 472.0,
        handicap: 119,
        polar_values: [[90.0, -0.550], [130.0, -0.792], [170.0, -1.425]],
    },
    BasicGliderData {
        // No 51,  imported from XCSoar
        name: "DG-800S (15m)",
        wing_area: 10.68,
        max_speed: 250.0,
        empty_mass: 260.0,
        max_ballast: 150.0,
        reference_weight: 370.0,
        handicap: 113,
        polar_values: [[92.1, -0.581], [130.0, -0.975], [170.0, -1.693]],
    },
    BasicGliderData {
        // No 52,  imported from XCSoar
        name: "DG-800S (18m)",
        wing_area: 11.81,
        max_speed: 250.0,
        empty_mass: 264.0,
        max_ballast: 150.0,
        reference_weight: 350.0,
        handicap: 119,
        polar_values: [[77.5, -0.473], [130.0, -0.926], [170.0, -1.795]],
    },
    BasicGliderData {
        // No 53,  imported from XCSoar
        name: "Delta USHPA-2",
        wing_area: 0.00,
        max_speed: 250.0,
        empty_mass: 5.0,
        max_ballast: 0.0,
        reference_weight: 100.0,
        handicap: 0,
        polar_values: [[30.0, -1.100], [44.3, -1.520], [58.0, -3.600]],
    },
    BasicGliderData {
        // No 54,  imported from XCSoar
        name: "Delta USHPA-3",
        wing_area: 0.00,
        max_speed: 250.0,
        empty_mass: 5.0,
        max_ballast: 0.0,
        reference_weight: 100.0,
        handicap: 0,
        polar_values: [[37.0, -0.950], [48.1, -1.150], [73.0, -3.600]],
    },
    BasicGliderData {
        // No 55,  imported from XCSoar
        name: "Delta USHPA-4",
        wing_area: 0.00,
        max_speed: 250.0,
        empty_mass: 5.0,
        max_ballast: 0.0,
        reference_weight: 100.0,
        handicap: 0,
        polar_values: [[37.0, -0.890], [48.3, -1.020], [76.5, -3.300]],
    },
    BasicGliderData {
        // No 56,  imported from XCSoar
        name: "Dimona",
        wing_area: 15.30,
        max_speed: 250.0,
        empty_mass: 470.0,
        max_ballast: 100.0,
        reference_weight: 670.0,
        handicap: 68,
        polar_values: [[100.0, -1.290], [120.0, -1.610], [150.0, -2.450]],
    },
    BasicGliderData {
        // No 57,  imported from XCSoar
        name: "Discus",
        wing_area: 10.58,
        max_speed: 250.0,
        empty_mass: 233.0,
        max_ballast: 182.0,
        reference_weight: 350.0,
        handicap: 107,
        polar_values: [[95.0, -0.630], [140.0, -1.230], [180.0, -2.290]],
    },
    BasicGliderData {
        // No 58,  imported from XCSoar
        name: "Discus 2b",
        wing_area: 10.60,
        max_speed: 250.0,
        empty_mass: 252.0,
        max_ballast: 200.0,
        reference_weight: 312.0,
        handicap: 108,
        polar_values: [[105.0, -0.660], [150.0, -1.050], [200.0, -2.000]],
    },
    BasicGliderData {
        // No 59,  imported from XCSoar
        name: "Discus 2c (18m)",
        wing_area: 11.36,
        max_speed: 250.0,
        empty_mass: 280.0,
        max_ballast: 188.0,
        reference_weight: 377.0,
        handicap: 114,
        polar_values: [[100.0, -0.570], [120.0, -0.760], [150.0, -1.330]],
    },
    BasicGliderData {
        // No 60,  imported from XCSoar
        name: "Duo Discus",
        wing_area: 16.40,
        max_speed: 250.0,
        empty_mass: 420.0,
        max_ballast: 80.0,
        reference_weight: 615.0,
        handicap: 112,
        polar_values: [[103.0, -0.640], [152.0, -1.250], [200.0, -2.510]],
    },
    BasicGliderData {
        // No 61,  imported from XCSoar
        name: "Duo Discus T",
        wing_area: 16.40,
        max_speed: 250.0,
        empty_mass: 445.0,
        max_ballast: 80.0,
        reference_weight: 615.0,
        handicap: 112,
        polar_values: [[103.0, -0.640], [152.0, -1.250], [200.0, -2.510]],
    },
    BasicGliderData {
        // No 62,  imported from XCSoar
        name: "Duo Discus xT",
        wing_area: 16.40,
        max_speed: 250.0,
        empty_mass: 445.0,
        max_ballast: 50.0,
        reference_weight: 700.0,
        handicap: 113,
        polar_values: [[110.0, -0.664], [155.0, -1.206], [200.0, -2.287]],
    },
    BasicGliderData {
        // No 63,  imported from XCSoar
        name: "EB 28",
        wing_area: 16.80,
        max_speed: 250.0,
        empty_mass: 570.0,
        max_ballast: 180.0,
        reference_weight: 670.0,
        handicap: 125,
        polar_values: [[100.0, -0.460], [120.0, -0.610], [150.0, -0.960]],
    },
    BasicGliderData {
        // No 64,  imported from XCSoar
        name: "EB 28 Edition",
        wing_area: 16.50,
        max_speed: 250.0,
        empty_mass: 570.0,
        max_ballast: 180.0,
        reference_weight: 670.0,
        handicap: 125,
        polar_values: [[100.0, -0.470], [120.0, -0.630], [150.0, -0.970]],
    },
    BasicGliderData {
        // No 65,  imported from XCSoar
        name: "G 102 Astir CS",
        wing_area: 12.40,
        max_speed: 250.0,
        empty_mass: 255.0,
        max_ballast: 90.0,
        reference_weight: 330.0,
        handicap: 96,
        polar_values: [[75.0, -0.700], [93.0, -0.740], [185.0, -3.100]],
    },
    BasicGliderData {
        // No 66,  imported from XCSoar
        name: "G 103 Twin 2",
        wing_area: 17.52,
        max_speed: 250.0,
        empty_mass: 390.0,
        max_ballast: 0.0,
        reference_weight: 580.0,
        handicap: 92,
        polar_values: [[99.0, -0.800], [175.0, -1.950], [225.0, -3.800]],
    },
    BasicGliderData {
        // No 67,  imported from XCSoar
        name: "G102 Club Astir",
        wing_area: 12.40,
        max_speed: 250.0,
        empty_mass: 255.0,
        max_ballast: 0.0,
        reference_weight: 380.0,
        handicap: 91,
        polar_values: [[75.0, -0.600], [100.0, -0.700], [180.0, -3.100]],
    },
    BasicGliderData {
        // No 68,  imported from XCSoar
        name: "G102 Std Astir",
        wing_area: 12.40,
        max_speed: 250.0,
        empty_mass: 260.0,
        max_ballast: 70.0,
        reference_weight: 380.0,
        handicap: 100,
        polar_values: [[75.0, -0.600], [100.0, -0.700], [180.0, -2.800]],
    },
    BasicGliderData {
        // No 69,  imported from XCSoar
        name: "G104 Speed Astir",
        wing_area: 11.50,
        max_speed: 250.0,
        empty_mass: 265.0,
        max_ballast: 90.0,
        reference_weight: 351.0,
        handicap: 105,
        polar_values: [[90.0, -0.630], [105.0, -0.720], [157.0, -2.000]],
    },
    BasicGliderData {
        // No 70,  imported from XCSoar
        name: "Genesis II",
        wing_area: 11.24,
        max_speed: 250.0,
        empty_mass: 240.0,
        max_ballast: 151.0,
        reference_weight: 374.0,
        handicap: 107,
        polar_values: [[94.0, -0.610], [141.1, -1.180], [172.4, -2.000]],
    },
    BasicGliderData {
        // No 71,  imported from XCSoar
        name: "Glasfluegel 304",
        wing_area: 9.90,
        max_speed: 250.0,
        empty_mass: 235.0,
        max_ballast: 145.0,
        reference_weight: 305.0,
        handicap: 110,
        polar_values: [[100.0, -0.780], [120.0, -0.970], [150.0, -1.430]],
    },
    BasicGliderData {
        // No 72,  imported from XCSoar
        name: "H-301 Libelle",
        wing_area: 9.80,
        max_speed: 250.0,
        empty_mass: 180.0,
        max_ballast: 50.0,
        reference_weight: 300.0,
        handicap: 100,
        polar_values: [[94.0, -0.680], [147.7, -2.030], [184.6, -4.100]],
    },
    BasicGliderData {
        // No 73,  imported from XCSoar
        name: "H201 Std Libelle",
        wing_area: 9.80,
        max_speed: 250.0,
        empty_mass: 185.0,
        max_ballast: 50.0,
        reference_weight: 304.0,
        handicap: 98,
        polar_values: [[97.0, -0.790], [152.4, -1.910], [190.5, -3.300]],
    },
    BasicGliderData {
        // No 74,  imported from XCSoar
        name: "H205 Club Libelle",
        wing_area: 9.80,
        max_speed: 250.0,
        empty_mass: 200.0,
        max_ballast: 0.0,
        reference_weight: 295.0,
        handicap: 96,
        polar_values: [[100.0, -0.850], [120.0, -1.210], [150.0, -2.010]],
    },
    BasicGliderData {
        // No 75,  imported from XCSoar
        name: "IS-28B2",
        wing_area: 18.24,
        max_speed: 250.0,
        empty_mass: 375.0,
        max_ballast: 0.0,
        reference_weight: 590.0,
        handicap: 84,
        polar_values: [[100.0, -0.820], [160.0, -2.280], [200.0, -4.270]],
    },
    BasicGliderData {
        // No 76,  imported from XCSoar
        name: "IS-29D2 Lark",
        wing_area: 10.40,
        max_speed: 250.0,
        empty_mass: 240.0,
        max_ballast: 0.0,
        reference_weight: 360.0,
        handicap: 96,
        polar_values: [[100.0, -0.820], [135.7, -1.550], [184.1, -3.300]],
    },
    BasicGliderData {
        // No 77,  imported from XCSoar
        name: "JS-1B (18m)",
        wing_area: 11.25,
        max_speed: 250.0,
        empty_mass: 310.0,
        max_ballast: 180.0,
        reference_weight: 405.0,
        handicap: 121,
        polar_values: [[108.0, -0.570], [152.0, -1.060], [180.0, -1.650]],
    },
    BasicGliderData {
        // No 78,  imported from XCSoar
        name: "JS-1C (21m)",
        wing_area: 12.25,
        max_speed: 250.0,
        empty_mass: 330.0,
        max_ballast: 180.0,
        reference_weight: 441.0,
        handicap: 126,
        polar_values: [[108.0, -0.520], [156.0, -1.100], [180.0, -1.620]],
    },
    BasicGliderData {
        // No 79,  imported from XCSoar
        name: "JS-3 (15m)",
        wing_area: 8.75,
        max_speed: 250.0,
        empty_mass: 270.0,
        max_ballast: 158.0,
        reference_weight: 350.0,
        handicap: 116,
        polar_values: [[100.0, -0.600], [130.0, -0.800], [160.0, -1.200]],
    },
    BasicGliderData {
        // No 80,  imported from XCSoar
        name: "JS-3 (18m)",
        wing_area: 9.95,
        max_speed: 250.0,
        empty_mass: 282.0,
        max_ballast: 158.0,
        reference_weight: 398.0,
        handicap: 122,
        polar_values: [[100.0, -0.550], [130.0, -0.720], [160.0, -1.120]],
    },
    BasicGliderData {
        // No 81,  imported from XCSoar
        name: "Janus (18m)",
        wing_area: 16.60,
        max_speed: 250.0,
        empty_mass: 405.0,
        max_ballast: 240.0,
        reference_weight: 498.0,
        handicap: 102,
        polar_values: [[100.0, -0.710], [120.0, -0.920], [150.0, -1.460]],
    },
    BasicGliderData {
        // No 82,  imported from XCSoar
        name: "Janus C FG",
        wing_area: 17.40,
        max_speed: 250.0,
        empty_mass: 405.0,
        max_ballast: 170.0,
        reference_weight: 603.0,
        handicap: 106,
        polar_values: [[115.5, -0.760], [171.8, -1.980], [210.0, -4.000]],
    },
    BasicGliderData {
        // No 83,  imported from XCSoar
        name: "Janus C RG",
        wing_area: 17.30,
        max_speed: 250.0,
        empty_mass: 405.0,
        max_ballast: 240.0,
        reference_weight: 519.0,
        handicap: 108,
        polar_values: [[90.0, -0.600], [120.0, -0.880], [160.0, -1.640]],
    },
    BasicGliderData {
        // No 84,  imported from XCSoar
        name: "Ka 2b",
        wing_area: 17.50,
        max_speed: 250.0,
        empty_mass: 278.0,
        max_ballast: 0.0,
        reference_weight: 418.0,
        handicap: 78,
        polar_values: [[87.0, -0.900], [120.0, -1.500], [150.0, -2.600]],
    },
    BasicGliderData {
        // No 85,  imported from XCSoar
        name: "Ka 4",
        wing_area: 16.34,
        max_speed: 250.0,
        empty_mass: 220.0,
        max_ballast: 0.0,
        reference_weight: 360.0,
        handicap: 54,
        polar_values: [[65.0, -0.950], [120.0, -2.500], [140.0, -3.500]],
    },
    BasicGliderData {
        // No 86,  imported from XCSoar
        name: "Ka 6CR",
        wing_area: 12.50,
        max_speed: 250.0,
        empty_mass: 185.0,
        max_ballast: 0.0,
        reference_weight: 310.0,
        handicap: 82,
        polar_values: [[64.8, -0.670], [130.0, -2.260], [170.0, -4.690]],
    },
    BasicGliderData {
        // No 87,  imported from XCSoar
        name: "Ka 6E",
        wing_area: 12.40,
        max_speed: 250.0,
        empty_mass: 185.0,
        max_ballast: 0.0,
        reference_weight: 310.0,
        handicap: 85,
        polar_values: [[87.3, -0.810], [141.9, -2.030], [174.7, -3.500]],
    },
    BasicGliderData {
        // No 88,  imported from XCSoar
        name: "Ka 7",
        wing_area: 17.50,
        max_speed: 250.0,
        empty_mass: 285.0,
        max_ballast: 0.0,
        reference_weight: 445.0,
        handicap: 78,
        polar_values: [[87.0, -0.920], [120.0, -1.550], [150.0, -2.700]],
    },
    BasicGliderData {
        // No 89,  imported from XCSoar
        name: "Ka 8",
        wing_area: 14.15,
        max_speed: 250.0,
        empty_mass: 190.0,
        max_ballast: 0.0,
        reference_weight: 290.0,
        handicap: 76,
        polar_values: [[74.1, -0.760], [101.9, -1.270], [166.7, -4.640]],
    },
    BasicGliderData {
        // No 90,  imported from XCSoar
        name: "L 33 Solo",
        wing_area: 11.00,
        max_speed: 250.0,
        empty_mass: 210.0,
        max_ballast: 0.0,
        reference_weight: 330.0,
        handicap: 86,
        polar_values: [[87.2, -0.800], [135.6, -1.730], [174.4, -3.400]],
    },
    BasicGliderData {
        // No 91,  imported from XCSoar
        name: "LAK-12",
        wing_area: 14.63,
        max_speed: 250.0,
        empty_mass: 360.0,
        max_ballast: 190.0,
        reference_weight: 430.0,
        handicap: 114,
        polar_values: [[75.0, -0.480], [125.0, -0.880], [175.0, -1.970]],
    },
    BasicGliderData {
        // No 92,  imported from XCSoar
        name: "LAK-17 (15m)",
        wing_area: 9.06,
        max_speed: 250.0,
        empty_mass: 220.0,
        max_ballast: 215.0,
        reference_weight: 285.0,
        handicap: 113,
        polar_values: [[100.0, -0.600], [120.0, -0.720], [150.0, -1.090]],
    },
    BasicGliderData {
        // No 93,  imported from XCSoar
        name: "LAK-17 (18m)",
        wing_area: 9.80,
        max_speed: 250.0,
        empty_mass: 225.0,
        max_ballast: 205.0,
        reference_weight: 295.0,
        handicap: 119,
        polar_values: [[100.0, -0.560], [120.0, -0.740], [150.0, -1.160]],
    },
    BasicGliderData {
        // No 94,  imported from XCSoar
        name: "LAK-19 (15m)",
        wing_area: 9.06,
        max_speed: 250.0,
        empty_mass: 220.0,
        max_ballast: 195.0,
        reference_weight: 285.0,
        handicap: 108,
        polar_values: [[100.0, -0.640], [120.0, -0.850], [150.0, -1.410]],
    },
    BasicGliderData {
        // No 95,  imported from XCSoar
        name: "LAK-19 (18m)",
        wing_area: 9.80,
        max_speed: 250.0,
        empty_mass: 226.0,
        max_ballast: 185.0,
        reference_weight: 295.0,
        handicap: 114,
        polar_values: [[100.0, -0.600], [120.0, -0.820], [150.0, -1.340]],
    },
    BasicGliderData {
        // No 96,  imported from XCSoar
        name: "LAK17a (15m)",
        wing_area: 9.06,
        max_speed: 250.0,
        empty_mass: 220.0,
        max_ballast: 180.0,
        reference_weight: 285.0,
        handicap: 113,
        polar_values: [[95.0, -0.574], [148.0, -1.310], [200.0, -2.885]],
    },
    BasicGliderData {
        // No 97,  imported from XCSoar
        name: "LAK17a (18m)",
        wing_area: 9.80,
        max_speed: 250.0,
        empty_mass: 225.0,
        max_ballast: 180.0,
        reference_weight: 298.0,
        handicap: 119,
        polar_values: [[115.0, -0.680], [158.0, -1.379], [200.0, -2.975]],
    },
    BasicGliderData {
        // No 98,  imported from XCSoar
        name: "LS-10s (15m)",
        wing_area: 10.27,
        max_speed: 250.0,
        empty_mass: 288.0,
        max_ballast: 170.0,
        reference_weight: 370.0,
        handicap: 113,
        polar_values: [[100.0, -0.640], [120.0, -0.800], [150.0, -1.260]],
    },
    BasicGliderData {
        // No 99,  imported from XCSoar
        name: "LS-10s (18m)",
        wing_area: 11.45,
        max_speed: 250.0,
        empty_mass: 295.0,
        max_ballast: 220.0,
        reference_weight: 380.0,
        handicap: 119,
        polar_values: [[100.0, -0.580], [120.0, -0.750], [150.0, -1.210]],
    },
    BasicGliderData {
        // No 100,  imported from XCSoar
        name: "LS-1c",
        wing_area: 9.74,
        max_speed: 250.0,
        empty_mass: 200.0,
        max_ballast: 91.0,
        reference_weight: 350.0,
        handicap: 98,
        polar_values: [[115.9, -1.020], [154.5, -1.840], [193.1, -3.300]],
    },
    BasicGliderData {
        // No 101,  imported from XCSoar
        name: "LS-1f",
        wing_area: 9.74,
        max_speed: 250.0,
        empty_mass: 230.0,
        max_ballast: 80.0,
        reference_weight: 345.0,
        handicap: 100,
        polar_values: [[100.0, -0.750], [120.0, -0.980], [150.0, -1.600]],
    },
    BasicGliderData {
        // No 102,  imported from XCSoar
        name: "LS-3",
        wing_area: 10.50,
        max_speed: 250.0,
        empty_mass: 270.0,
        max_ballast: 121.0,
        reference_weight: 383.0,
        handicap: 107,
        polar_values: [[93.0, -0.640], [127.0, -0.930], [148.2, -1.280]],
    },
    BasicGliderData {
        // No 103,  imported from XCSoar
        name: "LS-3 (17m)",
        wing_area: 11.22,
        max_speed: 250.0,
        empty_mass: 255.0,
        max_ballast: 0.0,
        reference_weight: 325.0,
        handicap: 109,
        polar_values: [[100.0, -0.610], [120.0, -0.840], [150.0, -1.530]],
    },
    BasicGliderData {
        // No 104,  self added
        name: "LS-3 WL",
        wing_area: 10.50,
        max_speed: 250.0,
        empty_mass: 295.0,
        max_ballast: 121.0,
        reference_weight: 396.0,
        handicap: 108,
        polar_values: [[80.0, -0.604], [105.0, -0.700], [180.0, -1.939]],
    },
    BasicGliderData {
        // No 105,  imported from XCSoar
        name: "LS-4",
        wing_area: 10.50,
        max_speed: 250.0,
        empty_mass: 235.0,
        max_ballast: 121.0,
        reference_weight: 361.0,
        handicap: 104,
        polar_values: [[100.0, -0.690], [120.0, -0.870], [150.0, -1.440]],
    },
    BasicGliderData {
        // No 106,  imported from XCSoar
        name: "LS-5",
        wing_area: 13.90,
        max_speed: 250.0,
        empty_mass: 361.0,
        max_ballast: 120.0,
        reference_weight: 461.0,
        handicap: 118,
        polar_values: [[75.0, -0.450], [135.0, -1.000], [172.5, -1.900]],
    },
    BasicGliderData {
        // No 107,  imported from XCSoar
        name: "LS-6 (15m)",
        wing_area: 10.53,
        max_speed: 250.0,
        empty_mass: 265.0,
        max_ballast: 160.0,
        reference_weight: 327.0,
        handicap: 111,
        polar_values: [[90.0, -0.600], [100.0, -0.658], [183.0, -1.965]],
    },
    BasicGliderData {
        // No 108,  imported from XCSoar
        name: "LS-6 (18m)",
        wing_area: 11.40,
        max_speed: 250.0,
        empty_mass: 272.0,
        max_ballast: 140.0,
        reference_weight: 330.0,
        handicap: 117,
        polar_values: [[90.0, -0.510], [100.0, -0.570], [183.0, -2.000]],
    },
    BasicGliderData {
        // No 109,  imported from XCSoar
        name: "LS-7wl",
        wing_area: 9.80,
        max_speed: 250.0,
        empty_mass: 235.0,
        max_ballast: 150.0,
        reference_weight: 350.0,
        handicap: 107,
        polar_values: [[103.8, -0.730], [155.7, -1.470], [180.0, -2.660]],
    },
    BasicGliderData {
        // No 110,  imported from XCSoar
        name: "LS-8 (15m)",
        wing_area: 10.50,
        max_speed: 250.0,
        empty_mass: 240.0,
        max_ballast: 185.0,
        reference_weight: 325.0,
        handicap: 108,
        polar_values: [[70.0, -0.510], [115.0, -0.850], [173.0, -2.000]],
    },
    BasicGliderData {
        // No 111,  imported from XCSoar
        name: "LS-8 (18m)",
        wing_area: 11.40,
        max_speed: 250.0,
        empty_mass: 250.0,
        max_ballast: 185.0,
        reference_weight: 325.0,
        handicap: 114,
        polar_values: [[80.0, -0.510], [94.0, -0.560], [173.0, -2.000]],
    },
    BasicGliderData {
        // No 112,  imported from XCSoar
        name: "Mini Nimbus",
        wing_area: 9.86,
        max_speed: 250.0,
        empty_mass: 235.0,
        max_ballast: 155.0,
        reference_weight: 345.0,
        handicap: 107,
        polar_values: [[100.0, -0.690], [120.0, -0.920], [150.0, -1.450]],
    },
    BasicGliderData {
        // No 113,  imported from XCSoar
        name: "Nimbus 2",
        wing_area: 14.41,
        max_speed: 250.0,
        empty_mass: 350.0,
        max_ballast: 159.0,
        reference_weight: 493.0,
        handicap: 114,
        polar_values: [[119.8, -0.750], [179.8, -2.140], [219.7, -3.800]],
    },
    BasicGliderData {
        // No 114,  imported from XCSoar
        name: "Nimbus 3",
        wing_area: 16.70,
        max_speed: 250.0,
        empty_mass: 396.0,
        max_ballast: 159.0,
        reference_weight: 527.0,
        handicap: 122,
        polar_values: [[116.2, -0.670], [174.3, -1.810], [232.4, -3.800]],
    },
    BasicGliderData {
        // No 115,  imported from XCSoar
        name: "Nimbus 3D",
        wing_area: 16.70,
        max_speed: 250.0,
        empty_mass: 485.0,
        max_ballast: 168.0,
        reference_weight: 712.0,
        handicap: 121,
        polar_values: [[93.6, -0.460], [175.4, -1.480], [218.7, -2.500]],
    },
    BasicGliderData {
        // No 116,  imported from XCSoar
        name: "Nimbus 3DM",
        wing_area: 16.70,
        max_speed: 250.0,
        empty_mass: 595.0,
        max_ballast: 168.0,
        reference_weight: 820.0,
        handicap: 121,
        polar_values: [[115.0, -0.570], [157.4, -0.980], [222.2, -2.300]],
    },
    BasicGliderData {
        // No 117,  imported from XCSoar
        name: "Nimbus 3T",
        wing_area: 16.70,
        max_speed: 250.0,
        empty_mass: 422.0,
        max_ballast: 310.0,
        reference_weight: 577.0,
        handicap: 121,
        polar_values: [[141.7, -0.990], [182.3, -1.890], [243.1, -4.000]],
    },
    BasicGliderData {
        // No 118,  imported from XCSoar
        name: "Nimbus 4",
        wing_area: 17.80,
        max_speed: 250.0,
        empty_mass: 470.0,
        max_ballast: 303.0,
        reference_weight: 597.0,
        handicap: 124,
        polar_values: [[85.1, -0.410], [128.0, -0.750], [162.7, -1.400]],
    },
    BasicGliderData {
        // No 119,  imported from XCSoar
        name: "Nimbus 4D",
        wing_area: 17.80,
        max_speed: 250.0,
        empty_mass: 515.0,
        max_ballast: 303.0,
        reference_weight: 743.0,
        handicap: 123,
        polar_values: [[107.5, -0.500], [142.7, -0.830], [181.5, -1.600]],
    },
    BasicGliderData {
        // No 120,  imported from XCSoar
        name: "Nimbus 4DM",
        wing_area: 17.80,
        max_speed: 250.0,
        empty_mass: 595.0,
        max_ballast: 168.0,
        reference_weight: 820.0,
        handicap: 123,
        polar_values: [[100.0, -0.480], [150.0, -0.870], [190.8, -1.600]],
    },
    BasicGliderData {
        // No 121,  imported from XCSoar
        name: "PIK-20B",
        wing_area: 10.00,
        max_speed: 250.0,
        empty_mass: 240.0,
        max_ballast: 144.0,
        reference_weight: 354.0,
        handicap: 102,
        polar_values: [[102.5, -0.690], [157.8, -1.590], [216.9, -3.600]],
    },
    BasicGliderData {
        // No 122,  imported from XCSoar
        name: "PIK-20D",
        wing_area: 10.00,
        max_speed: 250.0,
        empty_mass: 227.0,
        max_ballast: 144.0,
        reference_weight: 348.0,
        handicap: 104,
        polar_values: [[100.0, -0.690], [156.5, -1.780], [215.2, -4.200]],
    },
    BasicGliderData {
        // No 123,  imported from XCSoar
        name: "PIK-20E",
        wing_area: 10.00,
        max_speed: 250.0,
        empty_mass: 324.0,
        max_ballast: 80.0,
        reference_weight: 437.0,
        handicap: 104,
        polar_values: [[109.6, -0.830], [166.7, -2.000], [241.2, -4.700]],
    },
    BasicGliderData {
        // No 124,  imported from XCSoar
        name: "PIK-30M",
        wing_area: 10.63,
        max_speed: 250.0,
        empty_mass: 347.0,
        max_ballast: 0.0,
        reference_weight: 460.0,
        handicap: 0,
        polar_values: [[123.6, -0.780], [152.0, -1.120], [200.2, -2.200]],
    },
    BasicGliderData {
        // No 125,  imported from XCSoar
        name: "PW-5 Smyk",
        wing_area: 10.16,
        max_speed: 250.0,
        empty_mass: 190.0,
        max_ballast: 0.0,
        reference_weight: 300.0,
        handicap: 85,
        polar_values: [[99.5, -0.950], [158.5, -2.850], [198.1, -5.100]],
    },
    BasicGliderData {
        // No 126,  imported from XCSoar
        name: "PW-6",
        wing_area: 15.30,
        max_speed: 250.0,
        empty_mass: 360.0,
        max_ballast: 0.0,
        reference_weight: 546.0,
        handicap: 86,
        polar_values: [[104.0, -0.847], [152.0, -1.994], [200.0, -4.648]],
    },
    BasicGliderData {
        // No 127,  imported from XCSoar
        name: "Pegase 101A",
        wing_area: 10.50,
        max_speed: 250.0,
        empty_mass: 252.0,
        max_ballast: 120.0,
        reference_weight: 344.0,
        handicap: 102,
        polar_values: [[85.0, -0.620], [105.0, -0.750], [175.0, -2.540]],
    },
    BasicGliderData {
        // No 128,  imported from XCSoar
        name: "Phoebus C",
        wing_area: 14.06,
        max_speed: 250.0,
        empty_mass: 225.0,
        max_ballast: 150.0,
        reference_weight: 310.0,
        handicap: 100,
        polar_values: [[100.0, -0.700], [120.0, -0.980], [150.0, -1.580]],
    },
    BasicGliderData {
        // No 129,  imported from XCSoar
        name: "Pilatus B4 FG",
        wing_area: 14.00,
        max_speed: 250.0,
        empty_mass: 230.0,
        max_ballast: 0.0,
        reference_weight: 306.0,
        handicap: 86,
        polar_values: [[90.0, -0.847], [126.0, -1.644], [198.0, -5.098]],
    },
    BasicGliderData {
        // No 130,  imported from XCSoar
        name: "R-26S Gobe",
        wing_area: 18.00,
        max_speed: 250.0,
        empty_mass: 230.0,
        max_ballast: 0.0,
        reference_weight: 420.0,
        handicap: 0,
        polar_values: [[60.0, -1.020], [80.0, -0.960], [120.0, -2.110]],
    },
    BasicGliderData {
        // No 131,  imported from XCSoar
        name: "Russia AC-4",
        wing_area: 7.70,
        max_speed: 250.0,
        empty_mass: 140.0,
        max_ballast: 0.0,
        reference_weight: 250.0,
        handicap: 84,
        polar_values: [[99.3, -0.920], [140.0, -1.800], [170.0, -2.900]],
    },
    BasicGliderData {
        // No 132,  imported from XCSoar
        name: "SF-27B",
        wing_area: 13.00,
        max_speed: 250.0,
        empty_mass: 220.0,
        max_ballast: 0.0,
        reference_weight: 300.0,
        handicap: 88,
        polar_values: [[100.0, -0.810], [120.0, -1.270], [150.0, -2.500]],
    },
    BasicGliderData {
        // No 133,  imported from XCSoar
        name: "SGS 1-26E",
        wing_area: 14.87,
        max_speed: 250.0,
        empty_mass: 202.0,
        max_ballast: 0.0,
        reference_weight: 315.0,
        handicap: 63,
        polar_values: [[82.3, -1.040], [117.7, -1.880], [156.9, -3.800]],
    },
    BasicGliderData {
        // No 134,  imported from XCSoar
        name: "SGS 1-34",
        wing_area: 14.03,
        max_speed: 250.0,
        empty_mass: 259.0,
        max_ballast: 0.0,
        reference_weight: 354.0,
        handicap: 85,
        polar_values: [[89.8, -0.800], [143.7, -2.100], [179.6, -3.800]],
    },
    BasicGliderData {
        // No 135,  imported from XCSoar
        name: "SGS 1-35A",
        wing_area: 9.64,
        max_speed: 250.0,
        empty_mass: 180.0,
        max_ballast: 179.0,
        reference_weight: 381.0,
        handicap: 0,
        polar_values: [[98.7, -0.740], [151.8, -1.800], [202.9, -3.900]],
    },
    BasicGliderData {
        // No 136,  imported from XCSoar
        name: "SGS 1-36 Sprite",
        wing_area: 13.10,
        max_speed: 250.0,
        empty_mass: 215.0,
        max_ballast: 0.0,
        reference_weight: 322.0,
        handicap: 76,
        polar_values: [[76.0, -0.680], [133.0, -2.000], [170.9, -4.100]],
    },
    BasicGliderData {
        // No 137,  imported from XCSoar
        name: "SGS 2-33",
        wing_area: 20.35,
        max_speed: 250.0,
        empty_mass: 272.0,
        max_ballast: 0.0,
        reference_weight: 472.0,
        handicap: 54,
        polar_values: [[71.5, -0.960], [113.0, -1.740], [147.7, -3.440]],
    },
    BasicGliderData {
        // No 138,  imported from XCSoar
        name: "SZD-30 Pirat",
        wing_area: 13.80,
        max_speed: 250.0,
        empty_mass: 260.0,
        max_ballast: 0.0,
        reference_weight: 370.0,
        handicap: 86,
        polar_values: [[80.0, -0.720], [100.0, -0.980], [150.0, -2.460]],
    },
    BasicGliderData {
        // No 139,  imported from XCSoar
        name: "SZD-36 Cobra",
        wing_area: 11.60,
        max_speed: 250.0,
        empty_mass: 275.0,
        max_ballast: 30.0,
        reference_weight: 350.0,
        handicap: 98,
        polar_values: [[70.8, -0.600], [94.5, -0.690], [148.1, -1.830]],
    },
    BasicGliderData {
        // No 140,  imported from XCSoar
        name: "SZD-42 Jantar II",
        wing_area: 14.27,
        max_speed: 250.0,
        empty_mass: 362.0,
        max_ballast: 191.0,
        reference_weight: 482.0,
        handicap: 113,
        polar_values: [[109.5, -0.660], [157.1, -1.470], [196.4, -2.700]],
    },
    BasicGliderData {
        // No 141,  imported from XCSoar
        name: "SZD-48-2 Jantar",
        wing_area: 10.66,
        max_speed: 250.0,
        empty_mass: 274.0,
        max_ballast: 150.0,
        reference_weight: 375.0,
        handicap: 100,
        polar_values: [[100.0, -0.730], [120.0, -0.950], [150.0, -1.600]],
    },
    BasicGliderData {
        // No 142,  imported from XCSoar
        name: "SZD-48-3 Jantar",
        wing_area: 10.66,
        max_speed: 250.0,
        empty_mass: 274.0,
        max_ballast: 150.0,
        reference_weight: 326.0,
        handicap: 100,
        polar_values: [[95.0, -0.660], [180.0, -2.240], [220.0, -3.850]],
    },
    BasicGliderData {
        // No 143,  imported from XCSoar
        name: "SZD-50 Puchacz",
        wing_area: 18.16,
        max_speed: 250.0,
        empty_mass: 370.0,
        max_ballast: 135.0,
        reference_weight: 435.0,
        handicap: 84,
        polar_values: [[100.0, -1.000], [120.0, -1.420], [150.0, -2.350]],
    },
    BasicGliderData {
        // No 144,  imported from XCSoar
        name: "SZD-51-1 Junior",
        wing_area: 12.51,
        max_speed: 250.0,
        empty_mass: 242.0,
        max_ballast: 0.0,
        reference_weight: 333.0,
        handicap: 90,
        polar_values: [[70.0, -0.580], [130.0, -1.600], [180.0, -3.600]],
    },
    BasicGliderData {
        // No 145,  imported from XCSoar
        name: "SZD-54-2 17m",
        wing_area: 16.36,
        max_speed: 250.0,
        empty_mass: 375.0,
        max_ballast: 0.0,
        reference_weight: 442.0,
        handicap: 98,
        polar_values: [[98.0, -0.920], [174.0, -4.350], [250.0, -13.220]],
    },
    BasicGliderData {
        // No 146,  imported from XCSoar
        name: "SZD-54-2 17m WL",
        wing_area: 16.36,
        max_speed: 250.0,
        empty_mass: 375.0,
        max_ballast: 0.0,
        reference_weight: 442.0,
        handicap: 98,
        polar_values: [[99.0, -0.860], [175.0, -4.220], [250.0, -13.010]],
    },
    BasicGliderData {
        // No 147,  imported from XCSoar
        name: "SZD-54-2 20m WL",
        wing_area: 17.30,
        max_speed: 250.0,
        empty_mass: 380.0,
        max_ballast: 0.0,
        reference_weight: 442.0,
        handicap: 102,
        polar_values: [[91.0, -0.690], [170.0, -3.980], [250.0, -12.660]],
    },
    BasicGliderData {
        // No 148,  imported from XCSoar
        name: "SZD-55-1 Promyk",
        wing_area: 9.60,
        max_speed: 250.0,
        empty_mass: 215.0,
        max_ballast: 200.0,
        reference_weight: 350.0,
        handicap: 106,
        polar_values: [[100.0, -0.660], [120.0, -0.860], [150.0, -1.400]],
    },
    BasicGliderData {
        // No 149,  imported from XCSoar
        name: "SZD-9-1E Bocian",
        wing_area: 20.00,
        max_speed: 250.0,
        empty_mass: 330.0,
        max_ballast: 0.0,
        reference_weight: 540.0,
        handicap: 76,
        polar_values: [[70.0, -0.830], [90.0, -1.000], [140.0, -2.530]],
    },
    BasicGliderData {
        // No 150,  imported from XCSoar
        name: "Silene E78",
        wing_area: 18.00,
        max_speed: 250.0,
        empty_mass: 365.0,
        max_ballast: 0.0,
        reference_weight: 450.0,
        handicap: 0,
        polar_values: [[75.0, -0.548], [125.0, -1.267], [160.0, -2.439]],
    },
    BasicGliderData {
        // No 151,  imported from XCSoar
        name: "Skylark 4",
        wing_area: 16.10,
        max_speed: 250.0,
        empty_mass: 258.0,
        max_ballast: 0.0,
        reference_weight: 395.0,
        handicap: 84,
        polar_values: [[78.0, -0.637], [139.0, -2.000], [200.0, -5.092]],
    },
    BasicGliderData {
        // No 152,  imported from XCSoar
        name: "Std Cirrus",
        wing_area: 10.04,
        max_speed: 250.0,
        empty_mass: 215.0,
        max_ballast: 80.0,
        reference_weight: 337.0,
        handicap: 99,
        polar_values: [[93.2, -0.740], [149.2, -1.710], [205.1, -4.200]],
    },
    BasicGliderData {
        // No 153,  imported from XCSoar
        name: "Stemme S-10",
        wing_area: 18.70,
        max_speed: 250.0,
        empty_mass: 645.0,
        max_ballast: 0.0,
        reference_weight: 850.0,
        handicap: 110,
        polar_values: [[133.5, -0.830], [167.8, -1.410], [205.0, -2.300]],
    },
    BasicGliderData {
        // No 154,  imported from XCSoar
        name: "Taurus",
        wing_area: 12.26,
        max_speed: 250.0,
        empty_mass: 306.0,
        max_ballast: 0.0,
        reference_weight: 472.0,
        handicap: 99,
        polar_values: [[100.0, -0.710], [120.0, -0.830], [150.0, -1.350]],
    },
    BasicGliderData {
        // No 155,  imported from XCSoar
        name: "VSO-10 Gradient",
        wing_area: 12.00,
        max_speed: 250.0,
        empty_mass: 250.0,
        max_ballast: 0.0,
        reference_weight: 347.0,
        handicap: 96,
        polar_values: [[90.0, -0.780], [130.0, -1.410], [160.0, -2.440]],
    },
    BasicGliderData {
        // No 156,  imported from XCSoar
        name: "VT-116 Orlik II",
        wing_area: 12.80,
        max_speed: 250.0,
        empty_mass: 215.0,
        max_ballast: 0.0,
        reference_weight: 335.0,
        handicap: 86,
        polar_values: [[80.0, -0.700], [100.0, -1.050], [120.0, -1.650]],
    },
    BasicGliderData {
        // No 157,  self added
        name: "Ventus 2b 15m",
        wing_area: 9.70,
        max_speed: 250.0,
        empty_mass: 248.0,
        max_ballast: 200.0,
        reference_weight: 339.0,
        handicap: 115,
        polar_values: [[85.0, -0.576], [110.0, -0.648], [200.0, -2.230]],
    },
    BasicGliderData {
        // No 158,  imported from XCSoar
        name: "Ventus 2c 18m",
        wing_area: 11.03,
        max_speed: 250.0,
        empty_mass: 260.0,
        max_ballast: 180.0,
        reference_weight: 385.0,
        handicap: 120,
        polar_values: [[80.0, -0.500], [120.0, -0.730], [180.0, -2.000]],
    },
    BasicGliderData {
        // No 159,  imported from XCSoar
        name: "Ventus 2cT 18m",
        wing_area: 11.03,
        max_speed: 250.0,
        empty_mass: 305.0,
        max_ballast: 110.0,
        reference_weight: 410.0,
        handicap: 120,
        polar_values: [[100.0, -0.620], [150.0, -1.200], [200.0, -2.300]],
    },
    BasicGliderData {
        // No 160,  imported from XCSoar
        name: "Ventus 2cx 18m",
        wing_area: 11.03,
        max_speed: 250.0,
        empty_mass: 265.0,
        max_ballast: 215.0,
        reference_weight: 385.0,
        handicap: 120,
        polar_values: [[80.0, -0.500], [120.0, -0.730], [180.0, -2.000]],
    },
    BasicGliderData {
        // No 161,  imported from XCSoar
        name: "Ventus 2cxT 18m",
        wing_area: 11.03,
        max_speed: 250.0,
        empty_mass: 310.0,
        max_ballast: 130.0,
        reference_weight: 470.0,
        handicap: 120,
        polar_values: [[100.0, -0.560], [150.0, -1.130], [200.0, -2.280]],
    },
    BasicGliderData {
        // No 162,  imported from XCSoar
        name: "Ventus a/b 16.6m",
        wing_area: 9.96,
        max_speed: 250.0,
        empty_mass: 250.0,
        max_ballast: 151.0,
        reference_weight: 358.0,
        handicap: 113,
        polar_values: [[100.2, -0.640], [159.7, -1.470], [239.5, -4.300]],
    },
    BasicGliderData {
        // No 163,  imported from XCSoar
        name: "Ventus b (15m)",
        wing_area: 9.51,
        max_speed: 250.0,
        empty_mass: 240.0,
        max_ballast: 151.0,
        reference_weight: 341.0,
        handicap: 110,
        polar_values: [[97.7, -0.680], [156.3, -1.460], [234.4, -3.900]],
    },
    BasicGliderData {
        // No 164,  imported from XCSoar
        name: "Ventus cM (17.6)",
        wing_area: 10.14,
        max_speed: 250.0,
        empty_mass: 360.0,
        max_ballast: 0.0,
        reference_weight: 430.0,
        handicap: 115,
        polar_values: [[100.2, -0.600], [159.7, -1.320], [210.5, -2.500]],
    },
    BasicGliderData {
        // No 165,  imported from XCSoar
        name: "WA 26 P Squale",
        wing_area: 12.60,
        max_speed: 250.0,
        empty_mass: 228.0,
        max_ballast: 0.0,
        reference_weight: 330.0,
        handicap: 86,
        polar_values: [[80.0, -0.610], [152.0, -2.000], [174.0, -3.000]],
    },
    BasicGliderData {
        // No 166,  imported from XCSoar
        name: "Zuni II",
        wing_area: 10.13,
        max_speed: 250.0,
        empty_mass: 238.0,
        max_ballast: 182.0,
        reference_weight: 358.0,
        handicap: 0,
        polar_values: [[110.0, -0.880], [167.0, -2.210], [203.7, -3.600]],
    },
    BasicGliderData {
        // No 167,  Manufacturer's data interpreted by Andreas Westkamp
        name: "AS-33 18m",
        wing_area: 10.00,
        max_speed: 270.0,
        empty_mass: 285.0,
        max_ballast: 220.0,
        reference_weight: 400.0,
        handicap: 122,
        polar_values: [[97.2, -0.511], [111.6, -0.556], [180.0, -1.369]],
    },
    BasicGliderData {
        // No 168,  Manufacturer's data interpreted by Andreas Westkamp
        name: "AS-33 15m",
        wing_area: 8.80,
        max_speed: 270.0,
        empty_mass: 275.0,
        max_ballast: 180.0,
        reference_weight: 352.0,
        handicap: 116,
        polar_values: [[86.4, -0.583], [115.2, -0.642], [180.0, -1.473]],
    },
];
