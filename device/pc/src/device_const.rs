use corelib::{
    DeviceConst, DisplaySizes, HorizonSizes, HwVersion, Images, Misc, Palette, Sizes, SwVersion,
    VarioSizes,
};
use embedded_graphics::geometry::Point;
use u8g2_fonts::{fonts, FontRenderer};

pub const DISPLAY_HEIGHT: u32 = 480;
pub const DISPLAY_WIDTH: u32 = 480;

#[macro_export]
macro_rules! include_bytes_aligned {
    ($align_to:expr, $path:expr) => {{
        #[repr(C, align($align_to))]
        struct __Aligned<T: ?Sized>(T);
        const __DATA: &'static __Aligned<[u8]> = &__Aligned(*include_bytes!($path));
        &__DATA.0
    }};
}

pub const DEVICE_CONST: DeviceConst = DeviceConst {
    dark_theme: Palette::default(),
    bright_theme: Palette::default(),
    big_font: BIG_FONT,
    small_font: SMALL_FONT,
    images: images(),
    sizes: sizes(),
    misc: misc(),
};

const fn sizes() -> Sizes {
    Sizes {
        vario: vario_sizes(),
        horizon: horizon_sizes(),
        display: display_sizes(),
    }
}

const SW_VERSION: SwVersion = SwVersion {
    version: [0, 1, 1, 0],
};
const HW_VERSION: HwVersion = HwVersion {
    version: [1, 3, 1, 0],
};

const fn misc() -> Misc {
    Misc {
        sw_version: SW_VERSION,
        hw_version: HW_VERSION,
        uuid: 0x1234_5678,
    }
}

const fn display_sizes() -> DisplaySizes {
    DisplaySizes {
        height: 480,
        width: 480,
        margin: 2,
        radius: 238,
        center: Point::new(240, 240),
        screen_center: Point::new(240, 240),
    }
}

const fn horizon_sizes() -> HorizonSizes {
    HorizonSizes {
        t_width: 160,
        rm_len: 45,
        rm_width: 6.0,
        stroke_width: 3,
        box_height: 50,
        tc_pos_y: 375,
        tc_needle_y: 430,
        tc_needle_delta: -30,
        pitch_scale_len: 20,
    }
}

const fn vario_sizes() -> VarioSizes {
    VarioSizes {
        stf_diameter: 346,
        stf_width: 10,
        indicator_len: 71,
        indicator_width: 18,
        info_1_pos: Point::new(420, 200),
        pic_info_1_pos: Point::new(390, 125),
        mc_width: 0.14,
        mc_len: 33,
        tcr_width: 0.25,
        tcr_len: 33,
        glider_pos: Point::new(129, 205),
        north_pos: Point::new(216, 14),
        bat_pos: Point::new(375, 300),
        sat_pos: Point::new(410, 300),
        unit_pos: Point::new(208, 432),
        wind_pos: Point::new(280, 320),
        delta_pos: Point::new(247, 363),
        avg_climb_pos: Point::new(270, 105),
        version_pos: Point::new(300, 120),
        wind_len: 120,
        wind_len_min: 80,
        angle_m_s: 25.0,
        wp_vario_scale: [
            (338, 413, "5"),
            (261, 445, "4"),
            (178, 441, "3"),
            (104, 402, "2"),
            (54, 336, "1"),
            (36, 255, "0"),
            (54, 174, "1"),
            (104, 108, "2"),
            (178, 69, "3"),
            (261, 65, "4"),
            (338, 97, "5"),
        ],
    }
}

const fn images() -> Images {
    Images {
        bat_empty: include_bytes_aligned!(4, "../../../core/assets/size_480x480/bat_empty.lif"),
        bat_full: include_bytes_aligned!(4, "../../../core/assets/size_480x480/bat_full.lif"),
        bat_half: include_bytes_aligned!(4, "../../../core/assets/size_480x480/bat_half.lif"),
        glider: include_bytes_aligned!(4, "../../../core/assets/size_480x480/glider.lif"),
        north: include_bytes_aligned!(4, "../../../core/assets/size_480x480/north.lif"),
        spiral: include_bytes_aligned!(4, "../../../core/assets/size_480x480/spiral.lif"),
        straight: include_bytes_aligned!(4, "../../../core/assets/size_480x480/straight.lif"),
        km_h: include_bytes_aligned!(4, "../../../core/assets/size_480x480/km_h.lif"),
        m_s: include_bytes_aligned!(4, "../../../core/assets/size_480x480/m_s.lif"),
        sat: include_bytes_aligned!(4, "../../../core/assets/size_480x480/sat.lif"),
        wp_horizon: include_bytes_aligned!(4, "../../../core/assets/size_480x480/wp_horizon.lif"),
        wp_vario: include_bytes_aligned!(4, "../../../core/assets/size_480x480/wp_vario.lif"),
    }
}

pub const SMALL_FONT: FontRenderer = FontRenderer::new::<fonts::u8g2_font_fub17_tf>();
pub const BIG_FONT: FontRenderer = FontRenderer::new::<fonts::u8g2_font_fub30_tf>();

/*#[cfg(feature = "air_avionics_ad57")]
pub const SMALL_FONT: FontRenderer = FontRenderer::new::<fonts::u8g2_font_helvB14_tf>();
#[cfg(feature = "larus_frontend_v1")]
pub const SMALL_FONT: FontRenderer = FontRenderer::new::<fonts::u8g2_font_helvB14_tf>();
#[cfg(feature = "larus_frontend_v2")]
pub const SMALL_FONT: FontRenderer = FontRenderer::new::<fonts::u8g2_font_fub17_tf>();

#[cfg(feature = "air_avionics_ad57")]
pub const BIG_FONT: FontRenderer = FontRenderer::new::<fonts::u8g2_font_fub17_tf>();
#[cfg(feature = "larus_frontend_v1")]
pub const BIG_FONT: FontRenderer = FontRenderer::new::<fonts::u8g2_font_fub20_tf>();
#[cfg(feature = "larus_frontend_v2")]
pub const BIG_FONT: FontRenderer = FontRenderer::new::<fonts::u8g2_font_fub30_tf>();

#[macro_export]
macro_rules! include_bytes_aligned {
    ($align_to:expr, $path:expr) => {{
        #[repr(C, align($align_to))]
        struct __Aligned<T: ?Sized>(T);

        const __DATA: &'static __Aligned<[u8]> = &__Aligned(*include_bytes!($path));

        &__DATA.0
    }};
}

#[allow(clippy::module_inception)]
#[cfg(feature = "air_avionics_ad57")]
pub mod images {
    pub const BAT_EMPTY_IMG: &[u8] =
        include_bytes_aligned!(4, "../../../assets/size_227x285/bat_empty.lif");
    pub const BAT_FULL_IMG: &[u8] =
        include_bytes_aligned!(4, "../../../assets/size_227x285/bat_full.lif");
    pub const BAT_HALF_IMG: &[u8] =
        include_bytes_aligned!(4, "../../../assets/size_227x285/bat_half.lif");
    pub const GLIDER_IMG: &[u8] =
        include_bytes_aligned!(4, "../../../assets/size_227x285/glider.lif");
    pub const NORTH_IMG: &[u8] =
        include_bytes_aligned!(4, "../../../assets/size_227x285/north.lif");
    pub const SPIRAL_IMG: &[u8] =
        include_bytes_aligned!(4, "../../../assets/size_227x285/spiral.lif");
    pub const STRAIGHT_IMG: &[u8] =
        include_bytes_aligned!(4, "../../../assets/size_227x285/straight.lif");
    pub const KM_H_IMG: &[u8] = include_bytes_aligned!(4, "../../../assets/size_227x285/km_h.lif");
    pub const M_S_IMG: &[u8] = include_bytes_aligned!(4, "../../../assets/size_227x285/m_s.lif");
    pub const SAT_IMG: &[u8] = include_bytes_aligned!(4, "../../../assets/size_227x285/sat.lif");
    pub const WP_HORIZON_IMG: &[u8] =
        include_bytes_aligned!(4, "../../../assets/size_227x285/wp_horizon.lif");
    pub const WP_VARIO_IMG: &[u8] =
        include_bytes_aligned!(4, "../../../assets/size_227x285/wp_vario.lif");
}

#[allow(clippy::module_inception)]
#[cfg(feature = "larus_frontend_v1")]
pub mod images {
    use crate::include_bytes_aligned;
    pub const BAT_EMPTY_IMG: &[u8] =
        include_bytes_aligned!(4, "../../../assets/size_240x320/bat_empty.lif");
    pub const BAT_FULL_IMG: &[u8] =
        include_bytes_aligned!(4, "../../../assets/size_240x320/bat_full.lif");
    pub const BAT_HALF_IMG: &[u8] =
        include_bytes_aligned!(4, "../../../assets/size_240x320/bat_half.lif");
    pub const GLIDER_IMG: &[u8] =
        include_bytes_aligned!(4, "../../../assets/size_240x320/glider.lif");
    pub const NORTH_IMG: &[u8] =
        include_bytes_aligned!(4, "../../../assets/size_240x320/north.lif");
    pub const SPIRAL_IMG: &[u8] =
        include_bytes_aligned!(4, "../../../assets/size_240x320/spiral.lif");
    pub const STRAIGHT_IMG: &[u8] =
        include_bytes_aligned!(4, "../../../assets/size_240x320/straight.lif");
    pub const KM_H_IMG: &[u8] = include_bytes_aligned!(4, "../../../assets/size_240x320/km_h.lif");
    pub const M_S_IMG: &[u8] = include_bytes_aligned!(4, "../../../assets/size_240x320/m_s.lif");
    pub const SAT_IMG: &[u8] = include_bytes_aligned!(4, "../../../assets/size_240x320/sat.lif");
    pub const WP_HORIZON_IMG: &[u8] =
        include_bytes_aligned!(4, "../../../assets/size_240x320/wp_horizon.lif");
    pub const WP_VARIO_IMG: &[u8] =
        include_bytes_aligned!(4, "../../../assets/size_240x320/wp_vario.lif");
}

#[allow(clippy::module_inception)]
#[cfg(feature = "larus_frontend_v2")]
pub mod images {
    use crate::include_bytes_aligned;
    pub const BAT_EMPTY_IMG: &[u8] =
        include_bytes_aligned!(4, "../../../assets/size_480x480/bat_empty.lif");
    pub const BAT_FULL_IMG: &[u8] =
        include_bytes_aligned!(4, "../../../assets/size_480x480/bat_full.lif");
    pub const BAT_HALF_IMG: &[u8] =
        include_bytes_aligned!(4, "../../../assets/size_480x480/bat_half.lif");
    pub const GLIDER_IMG: &[u8] =
        include_bytes_aligned!(4, "../../../assets/size_480x480/glider.lif");
    pub const NORTH_IMG: &[u8] =
        include_bytes_aligned!(4, "../../../assets/size_480x480/north.lif");
    pub const SPIRAL_IMG: &[u8] =
        include_bytes_aligned!(4, "../../../assets/size_480x480/spiral.lif");
    pub const STRAIGHT_IMG: &[u8] =
        include_bytes_aligned!(4, "../../../assets/size_480x480/straight.lif");
    pub const KM_H_IMG: &[u8] = include_bytes_aligned!(4, "../../../assets/size_480x480/km_h.lif");
    pub const M_S_IMG: &[u8] = include_bytes_aligned!(4, "../../../assets/size_480x480/m_s.lif");
    pub const SAT_IMG: &[u8] = include_bytes_aligned!(4, "../../../assets/size_480x480/sat.lif");
    pub const WP_HORIZON_IMG: &[u8] =
        include_bytes_aligned!(4, "../../../assets/size_480x480/wp_horizon.lif");
    pub const WP_VARIO_IMG: &[u8] =
        include_bytes_aligned!(4, "../../../assets/size_480x480/wp_vario.lif");
}

struct VarioSizes {
    stf_diameter: u32,
    stf_width: u32,
    indicator_len: u32,
    indicator_width: u32,
    info_1_pos: Point,
    pic_info_1_pos: Point,
    mc_width: f32,
    mc_len: u32,
    tcr_width: f32,
    tcr_len: u32,
    glider_pos: Point,
    north_pos: Point,
    bat_pos: Point,
    sat_pos: Point,
    unit_pos: Point,
    wind_pos: Point,
    delta_pos: Point,
    avg_climb_pos: Point,
    version_pos: Point,
    wind_len: i32,
    wind_len_min: i32,
    angle_m_s: f32,
    wp_vario_scale: [(i32, i32, &'static str); 11],
}

#[cfg(feature = "air_avionics_ad57")]
const DIMS: VarioSizes = VarioSizes {
    stf_diameter: DIAMETER - 80,
    stf_width: 5,
    indicator_len: 37,
    indicator_width: 12,
    info_1_pos: Point::new(40, 258),
    pic_info_1_pos: Point::new(2, 222),
    mc_width: 0.14,
    mc_len: 22,
    tcr_width: 0.25,
    tcr_len: 22,
    glider_pos: Point::new(67, 118),
    north_pos: Point::new(127, 8),
    bat_pos: Point::new(205, 100),
    sat_pos: Point::new(10, 15),
    unit_pos: Point::new(122, 255),
    wind_pos: Point::new(175, 195),
    delta_pos: Point::new(150, 220),
    avg_climb_pos: Point::new(150, 65),
    version_pos: Point::new(200, 200),
    wind_len: 105,
    wind_len_min: 50,
    angle_m_s: 25.0,
    wp_vario_scale: [
        (202, 248, "5"),
        (156, 267, "4"),
        (105, 265, "3"),
        (61, 241, "2"),
        (30, 202, "1"),
        (20, 153, "0"),
        (30, 103, "1"),
        (61, 64, "2"),
        (105, 40, "3"),
        (156, 38, "4"),
        (202, 57, "5"),
    ],
};

#[cfg(feature = "larus_frontend_v1")]
const DIMS: VarioSizes = VarioSizes {
    stf_diameter: DIAMETER - 80,
    stf_width: 10,
    indicator_len: 45,
    indicator_width: 14,
    info_1_pos: Point::new(47, 290),
    pic_info_1_pos: Point::new(9, 254),
    mc_width: 0.16,
    mc_len: 25,
    tcr_width: 0.25,
    tcr_len: 25,
    glider_pos: Point::new(85, 136),
    north_pos: Point::new(135, 12),
    bat_pos: Point::new(215, 90),
    sat_pos: Point::new(10, 10),
    unit_pos: Point::new(130, 292),
    wind_pos: Point::new(195, 217),
    delta_pos: Point::new(147, 245),
    avg_climb_pos: Point::new(170, 70),
    version_pos: Point::new(200, 217),
    wind_len: 105,
    wind_len_min: 50,
    angle_m_s: 24.0,
    wp_vario_scale: [
        (217, 282, "5"),
        (166, 299, "4"),
        (112, 293, "3"),
        (65, 266, "2"),
        (34, 223, "1"),
        (23, 170, "0"),
        (34, 117, "1"),
        (65, 74, "2"),
        (112, 47, "3"),
        (166, 41, "4"),
        (217, 58, "5"),
    ],
};

#[cfg(feature = "larus_frontend_v2")]
const DIMS: VarioSizes = VarioSizes {
    stf_diameter: DIAMETER - 130,
    stf_width: 10,
    indicator_len: 71,
    indicator_width: 18,
    info_1_pos: Point::new(420, 200),
    pic_info_1_pos: Point::new(390, 125),
    mc_width: 0.14,
    mc_len: 33,
    tcr_width: 0.25,
    tcr_len: 33,
    glider_pos: Point::new(129, 205),
    north_pos: Point::new(216, 14),
    bat_pos: Point::new(375, 300),
    sat_pos: Point::new(410, 300),
    unit_pos: Point::new(208, 432),
    wind_pos: Point::new(280, 320),
    delta_pos: Point::new(247, 363),
    avg_climb_pos: Point::new(270, 105),
    version_pos: Point::new(300, 120),
    wind_len: 120,
    wind_len_min: 80,
    angle_m_s: 25.0,
    wp_vario_scale: [
        (338, 413, "5"),
        (261, 445, "4"),
        (178, 441, "3"),
        (104, 402, "2"),
        (54, 336, "1"),
        (36, 255, "0"),
        (54, 174, "1"),
        (104, 108, "2"),
        (178, 69, "3"),
        (261, 65, "4"),
        (338, 97, "5"),
    ],
};

#[cfg(feature = "air_avionics_ad57")]
mod c {
    pub const T_WIDTH: i32 = 88;
    pub const RM_LEN: i32 = 25;
    pub const RM_WIDTH: f32 = 7.0;
    pub const STROKE_WIDTH: i32 = 2;
    pub const BOX_HEIGHT: i32 = 30;
    pub const TC_POS_Y: i32 = super::DISPLAY_HEIGHT as i32 - 16;
    pub const TC_NEEDLE_Y: i32 = super::DISPLAY_WIDTH as i32;
    pub const TC_NEEDLE_DELTA: i32 = 18;
    pub const PITCH_SCALE_LEN: i32 = 18;
}

#[cfg(feature = "larus_frontend_v1")]
mod c {
    pub const T_WIDTH: i32 = 100;
    pub const RM_LEN: i32 = 30;
    pub const RM_WIDTH: f32 = 8.0;
    pub const STROKE_WIDTH: i32 = 2;
    pub const BOX_HEIGHT: i32 = 32;
    pub const TC_POS_Y: i32 = super::DISPLAY_HEIGHT as i32 - 30;
    pub const TC_NEEDLE_Y: i32 = super::DISPLAY_WIDTH as i32;
    pub const TC_NEEDLE_DELTA: i32 = 18;
    pub const PITCH_SCALE_LEN: i32 = 20;
}

#[cfg(feature = "larus_frontend_v2")]
mod c {
    pub const T_WIDTH: i32 = 160;
    pub const RM_LEN: i32 = 45;
    pub const RM_WIDTH: f32 = 6.0;
    pub const STROKE_WIDTH: i32 = 3;
    pub const BOX_HEIGHT: i32 = 50;
    pub const TC_POS_Y: i32 = super::DISPLAY_HEIGHT as i32 - 105;
    pub const TC_NEEDLE_Y: i32 = super::DISPLAY_WIDTH as i32 - 50;
    pub const TC_NEEDLE_DELTA: i32 = -30;
    pub const PITCH_SCALE_LEN: i32 = 20;
}

pub const MARGIN: i32 = 2;
pub const DIAMETER: u32 = DISPLAY_HEIGHT - 2 * MARGIN as u32;
pub const RADIUS: u32 = DIAMETER / 2;
pub const CENTER: Point = Point::new(RADIUS as i32 + MARGIN, RADIUS as i32 + MARGIN);
pub const SCREEN_CENTER: Point = Point::new(DISPLAY_WIDTH as i32 / 2, DISPLAY_HEIGHT as i32 / 2);




*/
