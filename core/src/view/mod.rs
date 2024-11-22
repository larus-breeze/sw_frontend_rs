use embedded_graphics::{draw_target::DrawTarget, geometry::Point};

pub mod edit;
pub(crate) mod helpers;
pub mod sw_update;

pub(crate) mod horizon;
pub(crate) mod vario;

use crate::{
    basic_config::{DISPLAY_HEIGHT, DISPLAY_WIDTH},
    model::{CoreModel, DisplayActive, EditMode},
    utils::Colors,
    view::{edit::Edit, horizon::Horizon, sw_update::SwUpdate, vario::Vario},
    CoreError, DrawImage,
};

#[allow(dead_code)]
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
}

#[cfg(feature = "air_avionics_ad57")]
const VARIO_SIZES: VarioSizes = VarioSizes {
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
};

#[cfg(feature = "larus_frontend_v1")]
const VARIO_SIZES: VarioSizes = VarioSizes {
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
};

#[cfg(feature = "larus_frontend_v2")]
const VARIO_SIZES: VarioSizes = VarioSizes {
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
};

// Debug build runs at 10 Hz
#[cfg(debug_assertions)]
pub const FRAME_RATE: u32 = 10;

// Release build runs at 30 Hz
#[cfg(not(debug_assertions))]
pub const FRAME_RATE: u32 = 20;

pub const MARGIN: i32 = 2;
pub const DIAMETER: u32 = DISPLAY_HEIGHT - 2 * MARGIN as u32;
pub const RADIUS: u32 = DIAMETER / 2;
pub const CENTER: Point = Point::new(RADIUS as i32 + MARGIN, RADIUS as i32 + MARGIN);
pub const SCREEN_CENTER: Point = Point::new(DISPLAY_WIDTH as i32 / 2, DISPLAY_HEIGHT as i32 / 2);

enum PrimaryView {
    Vario(Vario),
    Horizon(Horizon),
    SwUpade(SwUpdate),
}

enum SecondaryView {
    Edit(Edit),
}

pub struct CoreView<D>
where
    D: DrawTarget<Color = Colors, Error = CoreError> + DrawImage,
{
    pub display: D,
    primary_view: PrimaryView,
    secondary_view: Option<SecondaryView>,
    core_model: CoreModel,
}

impl<D> CoreView<D>
where
    D: DrawTarget<Color = Colors, Error = CoreError> + DrawImage,
{
    pub fn new(display: D, core_model: &CoreModel) -> Self {
        let primary_view = PrimaryView::Vario(Vario::new(core_model));
        let core_model = *core_model;
        CoreView {
            display,
            primary_view,
            secondary_view: None,
            core_model,
        }
    }

    pub fn prepare(&mut self, core_model: &CoreModel) {
        // take a snapshot
        self.core_model = *core_model;

        self.primary_view = match core_model.config.display_active {
            DisplayActive::Horizon => PrimaryView::Horizon(Horizon::new(core_model)),
            DisplayActive::FirmwareUpdate => PrimaryView::SwUpade(SwUpdate::new(core_model)),
            _ => PrimaryView::Vario(Vario::new(core_model)),
        };

        self.secondary_view = if core_model.control.editor.mode == EditMode::Section {
            Some(SecondaryView::Edit(Edit::new(core_model)))
        } else {
            None
        };
    }

    pub fn draw(&mut self) -> Result<(), CoreError> {
        match &self.primary_view {
            PrimaryView::Vario(vario) => vario.draw(&mut self.display, &self.core_model)?,
            PrimaryView::Horizon(horizon) => horizon.draw(&mut self.display, &self.core_model)?,
            PrimaryView::SwUpade(sw_update) => {
                sw_update.draw(&mut self.display, &self.core_model)?
            }
        }

        if let Some(secondary_view) = &self.secondary_view {
            match secondary_view {
                SecondaryView::Edit(edit) => edit.draw(&mut self.display, &self.core_model)?,
            }
        }

        Ok(())
    }
}
