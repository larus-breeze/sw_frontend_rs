use crate::{HW_VERSION, SW_VERSION};
use corelib::{DeviceConst, DisplaySizes, HorizonSizes, Images, Misc, Palette, Sizes, VarioSizes};
use embedded_graphics::geometry::{Point, Size};
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

const fn misc() -> Misc {
    Misc {
        sw_version: SW_VERSION,
        hw_version: HW_VERSION,
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
        m_s: Size::new(45, 29),
        km_h: Size::new(56, 36),

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
        info1_pos: Point::new(240, 120),
        info2_pos: Point::new(240, 360),
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
        bat_empty: include_bytes_aligned!(4, "../assets/bat_empty.lif"),
        bat_full: include_bytes_aligned!(4, "../assets/bat_full.lif"),
        bat_half: include_bytes_aligned!(4, "../assets/bat_half.lif"),
        glider: include_bytes_aligned!(4, "../assets/glider.lif"),
        north: include_bytes_aligned!(4, "../assets/north.lif"),
        spiral: include_bytes_aligned!(4, "../assets/spiral.lif"),
        straight: include_bytes_aligned!(4, "../assets/straight.lif"),
        km_h: include_bytes_aligned!(4, "../assets/km_h.lif"),
        m_s: include_bytes_aligned!(4, "../assets/m_s.lif"),
        sat: include_bytes_aligned!(4, "../assets/sat.lif"),
        wp_horizon: include_bytes_aligned!(4, "../assets/wp_horizon.lif"),
        wp_vario: include_bytes_aligned!(4, "../assets/wp_vario.lif"),
    }
}

pub const SMALL_FONT: FontRenderer = FontRenderer::new::<fonts::u8g2_font_fub17_tf>();
pub const BIG_FONT: FontRenderer = FontRenderer::new::<fonts::u8g2_font_fub30_tf>();
