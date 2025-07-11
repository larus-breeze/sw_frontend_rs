// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod adapt;
mod config;
mod error;
mod gui_events;
mod frontend;
mod hardware;
mod tcp;

mod device;
use device::dev_const as dev_const;

use adapt::Adapt;
pub use error::Error;
use gui_events::GuiEvents;
use corelib::*;
use frontend::Frontend;
use hardware::{SW_VERSION, HW_VERSION};

use std::{
    error::Error as StdError, path::PathBuf, sync::mpsc::channel
};

slint::include_modules!();

pub enum Com {
    None,
    Event(Event),
    TakeSnapshot,
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
