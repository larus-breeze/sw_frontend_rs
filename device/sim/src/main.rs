// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod adapt;
mod config;
mod error;
mod gui_events;
mod dev_const;
mod frontend;
mod hardware;
mod tcp;

use adapt::Adapt;
pub use error::Error;
use gui_events::GuiEvents;
use corelib::*;
use frontend::Frontend;

use std::{
    error::Error as StdError, path::PathBuf, sync::mpsc::channel
};

slint::include_modules!();

const SW_VERSION: SwVersion = SwVersion {
    version: [0, 1, 1, 0],
};
const HW_VERSION: HwVersion = HwVersion {
    version: [1, 3, 1, 0],
};

pub enum Com {
    None,
    Event(Event),
    SaveScreenshot(Option<PathBuf>),
    SaveScreenshotVarioPng,
    SaveScreenshotToClipboard,
    FilterNmeaIn(bool),
    FilterNmeaOut(bool),
    FilterIdleEvents(bool),
    FilterCanIn(bool),
    FilterCanOut(bool),
    FilterIncl(String),
    FilterExcl(String),
    LogWindowRun,
    LogWindowPause,
    LogWindowSave(Option<PathBuf>),
    LogWindowClear,
}


fn main() -> Result<(), Box<dyn StdError>> {
    let ui = AppWindow::new()?;
    let (tx_event , rx_event) = channel::<Com>();
    let adapt = Adapt::new(tx_event, &ui);

    Frontend::new(rx_event, &ui);
    GuiEvents::new(&adapt, &ui);
    ui.run()?;
    Ok(())
}
