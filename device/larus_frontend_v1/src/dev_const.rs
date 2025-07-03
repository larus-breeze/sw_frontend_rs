use crate::{HW_VERSION, SW_VERSION};
use corelib::{
    DeviceConst, DisplaySizes, EditMode, HorizonSizes, Images, Misc, Palette, Sizes, VarioSizes,
};
use embedded_graphics::{geometry::Point, prelude::Size};
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
        edit_mode: EditMode::Fullscreen,
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
        m_s: Size::new(29, 19),
        km_h: Size::new(37, 24),
        alarm: Size::new(67, 67),
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
        tc_needle_delta: 18,
        pitch_scale_len: 20,
    }
}

const fn vario_sizes() -> VarioSizes {
    VarioSizes {
        stf_diameter: 236,
        stf_width: 10,
        indicator_len: 45,
        attention_pos: Point::new(60, 100),
        glider_pos: Point::new(85, 136),
        north_pos: Point::new(135, 12),
        bat_pos: Point::new(215, 90),
        sat_pos: Point::new(10, 10),
        unit_pos: Point::new(130, 292),
        info1_pos: Point::new(160, 80),
        info2_pos: Point::new(160, 240),
        info3_pos: Point::new(47, 290),
        pic_info3_pos: Point::new(9, 254),
        small_gld_size: Size::new(30, 13),
        ta_circle_radius: 50,
        ta_point_diameter: 12,
        wind_len: 105,
        wind_len_min: 50,
        angle_m_s: 24.0,
    }
}

const fn images() -> Images {
    Images {
        attention: include_bytes_aligned!(4, "../assets/attention.lif"),
        bat_empty: include_bytes_aligned!(4, "../assets/bat_empty.lif"),
        bat_full: include_bytes_aligned!(4, "../assets/bat_full.lif"),
        bat_half: include_bytes_aligned!(4, "../assets/bat_half.lif"),
        gear: include_bytes_aligned!(4, "../assets/gear.lif"),
        glider: include_bytes_aligned!(4, "../assets/glider.lif"),
        north: include_bytes_aligned!(4, "../assets/north.lif"),
        spiral: include_bytes_aligned!(4, "../assets/spiral.lif"),
        straight: include_bytes_aligned!(4, "../assets/straight.lif"),
        km_h: include_bytes_aligned!(4, "../assets/km_h.lif"),
        m_s: include_bytes_aligned!(4, "../assets/m_s.lif"),
        sat: include_bytes_aligned!(4, "../assets/sat.lif"),
        small_glider: include_bytes_aligned!(4, "../assets/small_glider.lif"),
        wp_editor: include_bytes_aligned!(4, "../assets/wp_editor.lif"),
        wp_horizon: include_bytes_aligned!(4, "../assets/wp_horizon.lif"),
        wp_vario: include_bytes_aligned!(4, "../assets/wp_vario.lif"),
    }
}

pub const SMALL_FONT: FontRenderer = FontRenderer::new::<fonts::u8g2_font_helvB14_tf>();
pub const BIG_FONT: FontRenderer = FontRenderer::new::<fonts::u8g2_font_fub20_tf>();
