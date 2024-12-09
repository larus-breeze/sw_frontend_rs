

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
