use std::{path::PathBuf, sync::{Arc, Mutex}};
use corelib::*;
use std::sync::mpsc::Sender;
use crate::config::{Config, ConfigItem};

use super::{AppWindow, hardware, Com, InPins, LogSettings};
use slint::{ComponentHandle, Weak, quit_event_loop};
use serde::{Deserialize, Serialize};
use rfd::FileDialog;


#[derive(Deserialize, Serialize, Clone, PartialEq)]
#[serde(default)]
pub struct Settings {
    height: i32,
    width: i32,
    save_path: PathBuf,
}

impl Default for Settings {
    fn default() -> Self {
        let save_path = match std::env::current_dir() {
            Ok(path) => path,
            Err(_) => PathBuf::from("/"),
        };
        Settings {
            height: 670, 
            width: 850,
            save_path,
        }
    }
} 

#[derive(Deserialize, Serialize, Clone, PartialEq)]
#[serde(default)]
pub struct FilterSettings {
    nmea_in: bool,
    nmea_out: bool,
    idle_events: bool,
    can_in: bool,
    can_out: bool,
    filter_incl: String,
    filter_excl: String,
}

impl Default for FilterSettings {
    fn default() -> Self {
        FilterSettings {
            nmea_in: false,
            nmea_out: false,
            idle_events: false,
            can_in: false,
            can_out: false,
            filter_incl: String::new(),
            filter_excl: String::new(),
        }
        
    }
}

impl FilterSettings {
    fn init(&mut self, ui: &Weak<AppWindow>, com_tx_event: &Sender<Com>) {
        for cmd in ["\u{f70b}", "\u{f70c}", "\u{f70d}", "\u{f70e}", "\u{f70f}"] {
            self.set_ui(&ui, cmd);
            com_tx_event.send(self.get_com(cmd)).unwrap();
        }
        ui.upgrade().unwrap().global::<LogSettings>().set_le_excl((&self.filter_excl).into());
        com_tx_event.send(Com::FilterExcl(self.filter_excl.clone())).unwrap();
        ui.upgrade().unwrap().global::<LogSettings>().set_le_incl((&self.filter_incl).into());
        com_tx_event.send(Com::FilterIncl(self.filter_incl.clone())).unwrap();
    }

    fn set_filter_constrain(&mut self, src: &str, text: &str) -> Com {
        match src {
            "incl" => {
                self.filter_incl = String::from(text);
                Com::FilterIncl(String::from(text))
            }
            "excl" => {
                self.filter_excl = String::from(text);
                Com::FilterExcl(String::from(text))
            },
            _ => Com::None, 
        }
    }

    fn toggle(&mut self, cmd: &str) {
        match cmd {
            "\u{f70b}" => self.idle_events = ! self.idle_events,
            "\u{f70c}" => self.nmea_in = ! self.nmea_in,
            "\u{f70d}" => self.nmea_out = ! self.nmea_out,
            "\u{f70e}" => self.can_in = ! self.can_in,
            "\u{f70f}" => self.can_out = ! self.can_out,
            _ => (),
        }
    }

    fn get_com(&self, cmd: &str) -> Com {
        match cmd {
            "\u{f70b}" => Com::FilterIdleEvents(self.idle_events),
            "\u{f70c}" => Com::FilterNmeaIn(self.nmea_in),
            "\u{f70d}" => Com::FilterNmeaOut(self.nmea_out),
            "\u{f70e}" => Com::FilterCanIn(self.can_in),
            "\u{f70f}" => Com::FilterCanOut(self.can_out),
            _ => Com::None,
        }
    }

    fn set_ui(&self, ui: &Weak<AppWindow>, cmd: &str) {
        match cmd {
            "\u{f70b}" => ui.upgrade().unwrap().global::<LogSettings>().set_cb_idle_events(self.idle_events),
            "\u{f70c}" => ui.upgrade().unwrap().global::<LogSettings>().set_cb_nmea_in(self.nmea_in),
            "\u{f70d}" => ui.upgrade().unwrap().global::<LogSettings>().set_cb_nmea_out(self.nmea_out),
            "\u{f70e}" => ui.upgrade().unwrap().global::<LogSettings>().set_cb_can_in(self.can_in),
            "\u{f70f}" => ui.upgrade().unwrap().global::<LogSettings>().set_cb_can_out(self.can_out),
            _ => (),
        }
    }
}

struct AdaptInner {
    com_tx_event: Sender<Com>,
    in_pins: hardware::InPins,
    ui_handle: Weak<AppWindow>,
    config: Config,
    filter_settings: FilterSettings,
}

impl AdaptInner  {
    fn new(com_tx_event: Sender<Com>, ui: &AppWindow) -> Self {
        let config = Config::new();
        let ui_handle = ui.as_weak();
        if let ConfigItem::Adapt(settings) = config.get("adapt") {
            ui.set_app_preferred_height(settings.height);
            ui.set_app_preferred_width(settings.width);
        };

        let mut filter_settings = if let ConfigItem::Filter(settings) = config.get("filter") {
            settings.clone()
        } else { 
            panic!() 
        };
        filter_settings.init(&ui_handle, &com_tx_event);

        for cmd in ["\u{f70b}", "\u{f70c}", "\u{f70d}", "\u{f70e}", "\u{f70f}"] {
            filter_settings.set_ui(&ui_handle, cmd);
            com_tx_event.send(filter_settings.get_com(cmd)).unwrap();
        }

        let mut adapter = AdaptInner {
            com_tx_event,
            in_pins: hardware::InPins::new(),
            ui_handle,
            config: config,
            filter_settings
        };
        adapter.toggle_in_pin("1");
        adapter.toggle_in_pin("2");
        adapter.toggle_in_pin("3");
        adapter.toggle_in_pin("4");

        adapter
    }

    fn process_cmd(&mut self, cmd: &str) {
        let event = match cmd {
            "\u{f700}" => Com::Event(Event::KeyItem(KeyEvent::Rotary1Right)), 
            "\u{f701}" => Com::Event(Event::KeyItem(KeyEvent::Rotary1Left)), 
            "\u{f702}" => Com::Event(Event::KeyItem(KeyEvent::Rotary2Left)), 
            "\u{f703}" => Com::Event(Event::KeyItem(KeyEvent::Rotary2Right)),

            "\u{f704}" => Com::LogWindowRun,
            "\u{f705}" => Com::LogWindowPause,
            "\u{f706}" => {
                let path = self.get_save_file_path("log-file", &["txt", "log"]);
                Com::LogWindowSave(path)
            } 
            "\u{f707}" => Com::LogWindowClear,


            "\u{f708}" | "\n" => Com::Event(Event::KeyItem(KeyEvent::BtnEnc)), 
            "\u{f709}" => Com::Event(Event::KeyItem(KeyEvent::BtnEncS3)),
            "\u{f70a}" => {
                let path = self.get_save_file_path("picture", &["png"]);
                Com::SaveScreenshot(path)
            }
            "\u{f70b}" | "\u{f70c}" | "\u{f70d}" | "\u{f70e}" | "\u{f70f}" => {
                self.filter_settings.toggle(cmd);
                self.filter_settings.set_ui(&self.ui_handle, cmd);
                self.filter_settings.get_com(cmd)
            }


            "\u{1b}" => Com::Event(Event::KeyItem(KeyEvent::BtnEsc)),

            "1" | "2" | "3" | "4" => Com::Event(self.toggle_in_pin(cmd)),

            "c" => Com::SaveScreenshotToClipboard,

            "q" => {
                quit_event_loop().unwrap();
                Com::Event(Event::KeyItem(KeyEvent::NoEvent))
            },

            "s" => Com::SaveScreenshotVarioPng,

            _ => {
                println!("Key with no effect {:?}", cmd);
                Com::Event(Event::KeyItem(KeyEvent::NoEvent))
            },
        };
        self.com_tx_event.send(event).unwrap();
    }

    fn sec_tick(&mut self) {
        if let ConfigItem::Adapt(mut settings) = self.config.get("adapt") {
            settings.height = self.ui_handle.upgrade().unwrap().get_app_height() as i32;
            settings.width = self.ui_handle.upgrade().unwrap().get_app_width() as i32;
            self.config.set(ConfigItem::Adapt(settings));
        }
        self.config.set(ConfigItem::Filter(self.filter_settings.clone()));
    }

    fn toggle_in_pin(&mut self, cmd: &str) -> Event {
        self.in_pins.toggle(cmd);
        let text = self.in_pins.button_text(cmd);
        match cmd {
            "2" => self.ui_handle.upgrade().unwrap().global::<InPins>().set_in2_text(text.into()),
            "3" => self.ui_handle.upgrade().unwrap().global::<InPins>().set_in3_text(text.into()),
            "4" => self.ui_handle.upgrade().unwrap().global::<InPins>().set_in4_text(text.into()),
            _ => self.ui_handle.upgrade().unwrap().global::<InPins>().set_in1_text(text.into()),
        }
        self.in_pins.event(cmd)
    }

    fn set_filter_constrain(&mut self, src: &str, text: &str) {
        let com = self.filter_settings.set_filter_constrain(src, text);
        self.com_tx_event.send(com).unwrap();
    }

    fn get_save_file_path(&mut self, name: &str, extensions: &[&str]) -> Option<PathBuf> {
        // get settings
        let mut settings = if let ConfigItem::Adapt(settings) = self.config.get("adapt") {
            settings.clone()
        } else {
            panic!();
        };

        // get save_file_path
        let file_path = FileDialog::new()
            .add_filter(name, extensions)
            .set_directory(settings.save_path )
            .save_file();

        // save directory path if available
        match file_path.clone() {
            Some(file_path) => {
                match file_path.parent() {
                    Some(dir_path) => {
                        settings.save_path = PathBuf::from(dir_path);
                        self.config.set(ConfigItem::Adapt(settings));
                    }
                    None => (),
                }
            }
            None => (),
        }
        file_path
    }
}

pub struct Adapt {
    inner: Arc<Mutex<AdaptInner>>,
}

impl Adapt  {
    pub fn new(com_tx_event: Sender<Com>, ui: &AppWindow) -> Self {
        Adapt {
            inner: Arc::new(Mutex::new(AdaptInner::new(com_tx_event, ui)))
        }
    }

    pub fn process_cmd(&self, cmd: &str) {
        let mut inner = self.inner.lock().unwrap();
        inner.process_cmd(cmd);
    }

    pub fn sec_tick(&self) {
        let mut inner = self.inner.lock().unwrap();
        inner.sec_tick();
    }

    pub fn set_filter_constrain(&self, src: &str, text: &str) {
        let mut inner = self.inner.lock().unwrap();
        inner.set_filter_constrain(src, text);
    }

    pub fn clone(&self) -> Self {
        Adapt {
            inner: self.inner.clone(),
        }
    }
}

