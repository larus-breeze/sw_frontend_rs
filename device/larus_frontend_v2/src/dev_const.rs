use crate::{HW_VERSION, SW_VERSION};
use corelib::{
    DeviceConst, DisplaySizes, EditMode, HorizonSizes, Images, Misc, Palette, Sizes, VarioSizes,
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

const fn misc() -> Misc {
    Misc {
        sw_version: SW_VERSION,
        hw_version: HW_VERSION,
        edit_mode: EditMode::CircleArea,
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
        pitch_scale_len: 20,
        lt_swing_len: (DISPLAY_HEIGHT * 65 / 100) as i32,
        lt_mid: (DISPLAY_HEIGHT * 25 / 100) as i32,
        lt_rad: DISPLAY_WIDTH * 70 / 1000,
    }
}

const fn vario_sizes() -> VarioSizes {
    VarioSizes {
        stf_diameter: 346,
        stf_width: 10,
        indicator_len: 71,
        attention_pos: Point::new(90, 160),
        glider_pos: Point::new(129, 205),
        north_pos: Point::new(216, 14),
        bat_pos: Point::new(385, 130),
        sat_pos: Point::new(420, 130),
        profile_pos: Point::new(400, 175),
        unit_pos: Point::new(208, 432),
        info1_pos: Point::new(240, 120),
        info2_pos: Point::new(240, 360),
        info3_pos: Point::new(410, 280),
        pic_info3_pos: Point::new(380, 225),
        ta_circle_radius: 65,
        ta_point_diameter: 25,
        wind_len: 150,
        wind_len_min: 80,
        scale_factor: 25.0,
    }
}

const fn images() -> Images {
    Images {
        attention: include_bytes_aligned!(4, "../assets/attention.lif"),
        avg_climb_rate: include_bytes_aligned!(4, "../assets/avg_climb_rate.lif"),
        bat_empty: include_bytes_aligned!(4, "../assets/bat_empty.lif"),
        bat_full: include_bytes_aligned!(4, "../assets/bat_full.lif"),
        bat_half: include_bytes_aligned!(4, "../assets/bat_half.lif"),
        battery: include_bytes_aligned!(4, "../assets/battery.lif"),
        club_0: include_bytes_aligned!(4, "../assets/club-0.lif"),
        club_1: include_bytes_aligned!(4, "../assets/club-1.lif"),
        club_2: include_bytes_aligned!(4, "../assets/club-2.lif"),
        club_3: include_bytes_aligned!(4, "../assets/club-3.lif"),
        circle_delta: include_bytes_aligned!(4, "../assets/circle_delta.lif"),
        circle_diameter: include_bytes_aligned!(4, "../assets/circle_diameter.lif"),
        drift_angle: include_bytes_aligned!(4, "../assets/drift_angle.lif"),
        flight_level: include_bytes_aligned!(4, "../assets/flight_level.lif"),
        fpm: include_bytes_aligned!(4, "../assets/fpm.lif"),
        fpm_100: include_bytes_aligned!(4, "../assets/fpm-100.lif"),
        ft: include_bytes_aligned!(4, "../assets/ft.lif"),
        gear: include_bytes_aligned!(4, "../assets/gear.lif"),
        glider: include_bytes_aligned!(4, "../assets/glider.lif"),
        km_h: include_bytes_aligned!(4, "../assets/km_h.lif"),
        kt: include_bytes_aligned!(4, "../assets/kt.lif"),
        mph: include_bytes_aligned!(4, "../assets/mph.lif"),
        m: include_bytes_aligned!(4, "../assets/m.lif"),
        m_s: include_bytes_aligned!(4, "../assets/m_s.lif"),
        v: include_bytes_aligned!(4, "../assets/V.lif"),
        normal_0: include_bytes_aligned!(4, "../assets/normal-0.lif"),
        normal_1: include_bytes_aligned!(4, "../assets/normal-1.lif"),
        normal_2: include_bytes_aligned!(4, "../assets/normal-2.lif"),
        normal_3: include_bytes_aligned!(4, "../assets/normal-3.lif"),
        north: include_bytes_aligned!(4, "../assets/north.lif"),
        sat: include_bytes_aligned!(4, "../assets/sat.lif"),
        small_glider: include_bytes_aligned!(4, "../assets/small_glider.lif"),
        speed_to_fly: include_bytes_aligned!(4, "../assets/speed_to_fly.lif"),
        spiral: include_bytes_aligned!(4, "../assets/spiral.lif"),
        straight: include_bytes_aligned!(4, "../assets/straight.lif"),
        tas: include_bytes_aligned!(4, "../assets/tas.lif"),
        true_course: include_bytes_aligned!(4, "../assets/true_course.lif"),
        wp_editor: include_bytes_aligned!(4, "../assets/wp_editor.lif"),
        wp_horizon: include_bytes_aligned!(4, "../assets/wp_horizon.lif"),
        wp_vario_5: include_bytes_aligned!(4, "../assets/wp_vario-5.lif"),
        wp_vario_10: include_bytes_aligned!(4, "../assets/wp_vario-10.lif"),
    }
}

pub const SMALL_FONT: FontRenderer = FontRenderer::new::<fonts::u8g2_font_fub25_tf>();
pub const BIG_FONT: FontRenderer = FontRenderer::new::<fonts::u8g2_font_fub30_tf>();
