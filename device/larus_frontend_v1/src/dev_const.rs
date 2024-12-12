use crate::{HW_VERSION, SW_VERSION};
use corelib::{DeviceConst, DisplaySizes, HorizonSizes, Images, Misc, Palette, Sizes, VarioSizes};
use embedded_graphics::geometry::Point;
use u8g2_fonts::{fonts, FontRenderer};

pub const DISPLAY_HEIGHT: u32 = 320;
pub const DISPLAY_WIDTH: u32 = 240;

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
        height: 320,
        width: 240,
        margin: 2,
        radius: 158,
        center: Point::new(160, 160),
        screen_center: Point::new(120, 160),
    }
}

const fn horizon_sizes() -> HorizonSizes {
    HorizonSizes {
        t_width: 100,
        rm_len: 30,
        rm_width: 8.0,
        stroke_width: 2,
        box_height: 32,
        tc_pos_y: 290,
        tc_needle_y: 240,
        tc_needle_delta: -18,
        pitch_scale_len: 20,
    }
}

const fn vario_sizes() -> VarioSizes {
    VarioSizes {
        stf_diameter: 236,
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

pub const SMALL_FONT: FontRenderer = FontRenderer::new::<fonts::u8g2_font_helvB14_tf>();
pub const BIG_FONT: FontRenderer = FontRenderer::new::<fonts::u8g2_font_fub20_tf>();
