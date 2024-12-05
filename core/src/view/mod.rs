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
