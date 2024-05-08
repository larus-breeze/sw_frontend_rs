use embedded_graphics::prelude::*;

pub mod edit;
pub mod sw_update;

pub(crate) mod dialog_box;
pub(crate) mod elements;
pub(crate) mod vario;

use crate::{
    basic_config::{DISPLAY_HEIGHT, DISPLAY_WIDTH},
    model::{CoreModel, DisplayActive},
    utils::Colors,
    CoreError, DrawImage,
    view::{edit::Edit, sw_update::SwUpdate, vario::Vario},
};

#[allow(dead_code)]
struct VarioSizes {
    diameter_stf: u32,
    indicator_len: u32,
    indicator_width: u32,
    left_under_pos: Point,
    pic_left_under_pos: Point,
    mc_width: f32,
    mc_len: u32,
    tcr_width: f32,
    tcr_len: u32,
    bat_pos: Point,
    sat_pos: Point,
    unit_pos: Point,
    wind_pos: Point,
    version_pos: Point,
    wind_len: i32,
    wind_len_min: i32,
    angle_m_s: f32,
}

#[cfg(feature = "air_avionics_ad57")]
const VARIO_SIZES: VarioSizes = VarioSizes {
    diameter_stf: DIAMETER - 108,
    indicator_len: 50,
    indicator_width: 8,
    left_under_pos: Point::new(40, 258),
    pic_left_under_pos: Point::new(2, 222),
    mc_width: 0.14,
    mc_len: 22,
    tcr_width: 0.25,
    tcr_len: 22,
    bat_pos: Point::new(207, 70),
    sat_pos: Point::new(5, 40),
    unit_pos: Point::new(122, 255),
    wind_pos: Point::new(180, 200),
    version_pos: Point::new(200, 200),
    wind_len: 105,
    wind_len_min: 50,
    angle_m_s: 25.0,
};

#[cfg(feature = "larus_ad57")]
const VARIO_SIZES: VarioSizes = VarioSizes {
    diameter_stf: DIAMETER - 108,
    indicator_len: 50,
    indicator_width: 8,
    left_under_pos: Point::new(47, 290),
    pic_left_under_pos: Point::new(9, 254),
    mc_width: 0.14,
    mc_len: 22,
    tcr_width: 0.25,
    tcr_len: 22,
    bat_pos: Point::new(215, 70),
    sat_pos: Point::new(5, 33),
    unit_pos: Point::new(130, 285),
    wind_pos: Point::new(195, 217),
    version_pos: Point::new(200, 217),
    wind_len: 105,
    wind_len_min: 50,
    angle_m_s: 24.0,
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
    primary_view: Option<PrimaryView>,
    secondary_view: Option<SecondaryView>,
}

impl<D> CoreView<D>
where
    D: DrawTarget<Color = Colors, Error = CoreError> + DrawImage,
{
    pub fn new(display: D) -> Self {
        CoreView { display, primary_view: None, secondary_view: None }
    }

    pub fn prepare(&mut self, core_model: &CoreModel) {
        self.primary_view = match core_model.config.display_active {
            DisplayActive::Vario => Some(PrimaryView::Vario(Vario::new(core_model))),
            DisplayActive::FirmwareUpdate => Some(PrimaryView::SwUpade(SwUpdate::preapare(&core_model))),
        };

        self.secondary_view = if core_model.control.edit_ticks > 0 {
            Some(SecondaryView::Edit(Edit::new(core_model)))
        } else {
            None
        };
    }

    pub fn draw(&mut self) -> Result<(), CoreError> {
        if let Some(primary_view) = &self.primary_view {
            match primary_view {
                PrimaryView::Vario(vario) => vario.draw(&mut self.display)?,
                PrimaryView::SwUpade(sw_update) => sw_update.draw(&mut self.display)?,
            }
        }

        if let Some(secondary_view) = &self.secondary_view {
            match secondary_view {
                SecondaryView::Edit(edit) => edit.draw(&mut self.display)?,
            } 
        }

        Ok(())
    }
}
