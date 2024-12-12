use crate::{HW_VERSION, SW_VERSION};
use corelib::{DeviceConst, DisplaySizes, HorizonSizes, Images, Misc, Palette, Sizes, VarioSizes};
use embedded_graphics::geometry::Point;
use u8g2_fonts::{fonts, FontRenderer};

pub const DISPLAY_HEIGHT: u32 = 285;
pub const DISPLAY_WIDTH: u32 = 227;

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
        height: 285,
        width: 227,
        margin: 2,
        radius: 140,
        center: Point::new(142, 142),
        screen_center: Point::new(113, 142),
    }
}

const fn horizon_sizes() -> HorizonSizes {
    HorizonSizes {
        t_width: 88,
        rm_len: 25,
        rm_width: 7.0,
        stroke_width: 2,
        box_height: 30,
        tc_pos_y: 269,
        tc_needle_y: 2227,
        tc_needle_delta: 18,
        pitch_scale_len: 18,
    }
}

const fn vario_sizes() -> VarioSizes {
    VarioSizes {
        stf_diameter: 201,
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
