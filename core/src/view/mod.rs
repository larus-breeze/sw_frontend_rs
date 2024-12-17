use embedded_graphics::draw_target::DrawTarget;

pub mod editor;
pub mod fw_update;
pub mod helpers;

pub(crate) mod horizon;
pub(crate) mod menu;
pub(crate) mod vario;
pub(crate) mod viewable;

use crate::{
    model::{CoreModel, DisplayActive, EditMode},
    utils::Colors,
    view::{editor::Edit, fw_update::SwUpdate, horizon::Horizon, menu::MenuView, vario::Vario},
    CoreError, DrawImage,
};

// Debug build runs at 10 Hz
#[cfg(debug_assertions)]
pub const FRAME_RATE: u32 = 10;

// Release build runs at 30 Hz
#[cfg(not(debug_assertions))]
pub const FRAME_RATE: u32 = 20;

enum PrimaryView {
    Vario(Vario),
    Horizon(Horizon),
    SwUpade(SwUpdate),
    MenuView(MenuView),
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
        let primary_view = PrimaryView::Vario(Vario::new());
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

        // set the orientation
        self.display.set_rotation(core_model.control.rotation);

        self.primary_view = match core_model.config.display_active {
            DisplayActive::Horizon => PrimaryView::Horizon(Horizon::new()),
            DisplayActive::FirmwareUpdate => {
                let update_state = core_model.control.firmware_update_state;
                PrimaryView::SwUpade(SwUpdate::new(update_state))
            }
            DisplayActive::Menu => PrimaryView::MenuView(MenuView::new()),
            _ => PrimaryView::Vario(Vario::new()),
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
            PrimaryView::MenuView(menu_view) => {
                menu_view.draw(&mut self.display, &self.core_model)?
            }
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
