use std::{
    collections::VecDeque, fs::File, io::Write, path::PathBuf, str::FromStr, sync::{Arc, Mutex}, thread, vec
};

use corelib::*;
use slint::{ComponentHandle, Image, Rgba8Pixel, SharedPixelBuffer, Weak, quit_event_loop};
use heapless::spsc::Queue;
use arboard::{Clipboard, ImageData};

use crate::{
    Com, tcp::TcpServer, hardware, OutPins, LogSettings, Error,
    dev_const::{DEVICE_CONST, DISPLAY_WIDTH, DISPLAY_HEIGHT},
    AppWindow, hardware::{CanReader, Display, Storage, Sound},
    hardware::{DISPLAY_WIDTH_INC_PAD, DISPLAY_HEIGHT_INC_PAD},
};

use super::Lcd;

fn millis() -> u16 {
    let since_the_epoch = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Time went backwards");
    since_the_epoch.as_millis() as u16
}

pub struct Outputs(pub Arc<Mutex<hardware::OutPins>>);

impl Outputs {
    pub fn new() -> Self {
        Outputs(Arc::new(Mutex::new(hardware::OutPins::new())))
    }

    pub fn set_state(&self, ui_handle: &Weak<AppWindow>, event: IdleEvent) {
        let mut ouputs = self.0.lock().unwrap();
        let text = match ouputs.set_state(event) {
            Some(text) => text,
            None => return,
        };
        match event {
            IdleEvent::Output1(_) => ui_handle.upgrade().unwrap().global::<OutPins>().set_out1_text(text.into()),
            IdleEvent::Output2(_) => ui_handle.upgrade().unwrap().global::<OutPins>().set_out2_text(text.into()),
            _ => return,
        }
    }

    pub fn clone(&self) -> Self {
        Outputs(self.0.clone())
    }

}

const DISPLAY_LINES: usize = 25;
const DISPLAY_CHARS: usize = 60;

struct Logger {
    content: Vec<String>,
    pause: bool,
    display_string: String,
    filter_excl: String,
    filter_incl: String,
}

impl Logger {
    fn new() -> Self {
        Logger { 
            content: Vec::new(),
            pause: false,
            display_string: String::new(),
            filter_excl: String::new(),
            filter_incl: String::new(),
        }
    }

    fn add_from_bytes(&mut self, msg: &[u8]) {
        let s = match String::from_utf8(msg.to_vec()) {
            Ok(s) => s,
            Err(_) => String::from("<UTF8 Error>")
        };
        self.add(&s)
    }

    fn add(&mut self, msg: &str) {
        for sub in msg.split('\n') {
            self.add_sub(sub);
        }
    }

    fn add_sub(&mut self, msg: &str) {
        if msg.len() == 0 {
            return
        }

        for excl_pattern in self.filter_excl.split('@') {
            if excl_pattern != "" {
                if msg.contains(excl_pattern) {
                    return
                }
            }
        }

        if self.filter_incl.len() > 0 {
            let mut included = false;
            for incl_pattern in self.filter_incl.split('@') {
                if incl_pattern != "" {
                    if msg.contains(incl_pattern) {
                        included = true
                    }
                }
            }
            if !included {
                return
            }
        }

        self.content.push(String::from_utf8(msg.into()).unwrap());
    }

    fn run(&mut self) {
        self.pause = false;
    }

    fn pause(&mut self) {
        self.pause = true;
    }

    fn clear(&mut self) {
        self.content = Vec::new();
        self.display_string = String::new();
    }

    fn save(&mut self, file_path: Option<PathBuf>) {
        match file_path {
            Some(file_path) => {
                match self.write_to_file(&file_path) {
                    Ok(_) => (),
                    Err(_) => eprintln!("Could not write to '{:?}'", file_path),
                }
            }
            None => (),
        }
    }

    fn set_incl_filter(&mut self, text: String) {
        self.filter_incl = text;
    }

    fn set_excl_filter(&mut self, text: String) {
        self.filter_excl = text;
    }

    fn write_to_file(&self, file_path: &PathBuf) -> Result<(), Error> {
        let mut output = File::create(file_path).map_err(|_| Error::FileIo)?;
        for s in &self.content {
            write!(output, "{}\n", s).map_err(|_| Error::FileIo)?;
        }
        Ok(())
    }

    fn to_display(&mut self) -> String {
        if !self.pause {
            let len = self.content.len();
            let mut cnt = if len > DISPLAY_LINES {
                DISPLAY_LINES
            } else {
                len
            };
            let mut result = String::new();
            while cnt > 0 {
                let mut s = self.content[len-cnt].clone();
                if s.len() > DISPLAY_CHARS {
                    s = String::from_str(&s[0..DISPLAY_CHARS]).unwrap() + "...";
                }
                result += &s;
                if cnt > 1 {
                    result += "\n";
                }
                cnt -= 1
            }
            self.display_string = result;
        }
        self.display_string.clone()
    }
}


pub struct Frontend {}

impl Frontend {
    pub fn new(com_rx_event: std::sync::mpsc::Receiver<Com>, ui: &AppWindow) {
        let frontend = Frontend { };

        let fe_thread = thread::Builder::new().stack_size(100_000_000);
        fe_thread.spawn(frontend.run(ui.as_weak(), com_rx_event)).unwrap();    
    }

    pub fn run(
        self, 
        ui_handle_: Weak<AppWindow>, 
        com_rx_event: std::sync::mpsc::Receiver<Com>
    ) -> impl FnOnce() {
        move || {
            let mut logger = Logger::new();
            let mut filter_nmea_in = false;
            let mut filter_nmea_out = false;
            let mut filter_idle_events = false;
            let mut filter_can_in = false;
            let mut filter_can_out = false;

            let display = Display::new();
            let (p_idle_events, mut c_idle_events) = spsc_queue!(QIdleEvents);
            let (p_tx_frames, mut c_tx_frames) = spsc_queue!(QTxFrames<10>);
        
            let mut cm = CoreModel::new(&DEVICE_CONST, 0x1234_5678);
            let mut cc = CoreController::new(&mut cm, p_idle_events, p_tx_frames);
            let mut view = CoreView::new(display, &cm);

            let mut eeprom_init_items = VecDeque::<PersistenceItem>::new();
            let mut eeprom = Storage::new().unwrap();
            for item in eeprom.iter_over(EepromTopic::ConfigValues) {
                persist::restore_item(&mut cc, &mut cm, item);
                let _ = eeprom_init_items.push_back(item);
            }
        
            let outputs_ = Outputs::new();
            let mut can_reader = CanReader::new("127.0.0.1:5005");
            let sound = Sound::new();
            let mut img_no = 0;
            let mut clipboard_ctx = Clipboard::new().unwrap();

            let mut nmea_server = TcpServer::new("127.0.0.1:4353");
            let start_time = millis();

            loop {
                std::thread::sleep(std::time::Duration::from_millis(30));

                while let Ok(com) = com_rx_event.try_recv() {
                    match com {
                        Com::None => (),
                        Com::Event(event) => {
                            cc.event_handler(event, &mut cm);
                        }
                        Com::SaveScreenshotVarioPng => {
                            let img_path = format!("vario_{:03}.png", img_no);
                            view.display.save(Some(PathBuf::from(&img_path)));
                            img_no += 1;
                        }
                        Com::TakeSnapshot => view.display.take_snapshot(),
                        Com::SaveScreenshot(path) => view.display.save(path),
                        Com::SaveScreenshotToClipboard => {
                            let image = view.display.image_buffer();
                            let img_data  = ImageData{ 
                                width: DISPLAY_WIDTH_INC_PAD as usize, 
                                height: DISPLAY_HEIGHT_INC_PAD as usize, 
                                bytes: image.as_raw().into() 
                            };
                            clipboard_ctx.set_image(img_data).unwrap();
                        }
                        Com::FilterNmeaOut(state) => filter_nmea_out = state,
                        Com::FilterNmeaIn(state) => filter_nmea_in = state,
                        Com::FilterIdleEvents(state) => filter_idle_events = state,
                        Com::FilterCanIn(state) => filter_can_in = state,
                        Com::FilterCanOut(state) => filter_can_out = state,
                        Com::FilterExcl(text) => logger.set_excl_filter(text),
                        Com::FilterIncl(text) => logger.set_incl_filter(text),

                        Com::LogWindowRun => logger.run(),
                        Com::LogWindowPause => logger.pause(),
                        Com::LogWindowSave(path) => logger.save(path),
                        Com::LogWindowClear => logger.clear(),
                    }
                };

                while let Some(frame) = can_reader.read() {
                    if filter_can_in {
                        let msg = format!("{:?}", frame.basic_frame());
                        logger.add(&msg);
                    }
                    cc.read_can_frame(&mut cm, &frame);       
                }

                while let Some(nmea_data) = cc.nmea_next(&mut cm) {
                    nmea_server.send(nmea_data);
                    if filter_nmea_out {
                        logger.add_from_bytes(nmea_data);
                    }
                }

                if let Some(rx_data) = nmea_server.recv() {
                    cc.nmea_recv_slice(&mut cm, rx_data.as_slice());
                    println!("nmea in {}", filter_nmea_in);
                    if filter_nmea_in {
                        logger.add_from_bytes(&rx_data);
                    }
                }
        
                cc.tick_1ms(millis().wrapping_sub(start_time), &mut cm);
                view.prepare(&cm);
                view.draw().unwrap();

                let buf = view.display.copy();
                let buffer = SharedPixelBuffer::<Rgba8Pixel>::clone_from_slice(
                    &buf,
                    DISPLAY_WIDTH,
                    DISPLAY_HEIGHT,
                );

                while c_tx_frames.len() > 0 {
                    let frame = c_tx_frames.dequeue().unwrap();
                    if let Frame::Specific(specific_frame) = frame {
                        if specific_frame.specific_id == 0 {
                            sound.from_can_datagram(frame);
                        }
                    }
                    if filter_can_out {
                        let msg = format!("{:?}", frame);
                        logger.add(&msg);
                    }
                }

                while eeprom_init_items.len() > 0 {
                    let item = eeprom_init_items.pop_front().unwrap();
                    if filter_idle_events {
                        let msg = format!("{:?}", item);
                        logger.add(&msg);
                    }
                }
        
                let mut ui_events = vec::Vec::<IdleEvent>::new();
                while c_idle_events.len() > 0 {
                    let event = c_idle_events.dequeue().unwrap(); 
                    match event {
                        IdleEvent::Output1(_) | IdleEvent::Output2(_) => ui_events.push(event),
                        IdleEvent::SetEepromItem(item) => {
                            eeprom.write_item(item).unwrap();
                        }
                        IdleEvent::ClearEepromItems(items_list) => {
                            eeprom.delete_items_list(items_list).unwrap();
                        }
                        IdleEvent::ResetDevice(reason) => {
                            println!("Reset triggered by app, reason ‘{:?}’, please restart", reason);
                            quit_event_loop().unwrap();
                        } 
        
                        //...
                        _ => (),
                    }
                    if filter_idle_events {
                        let msg = format!("{:?}", event);
                        logger.add(&msg);
                    }
                }

                let ui_handle = ui_handle_.clone();
                let outputs = outputs_.clone();
                slint::invoke_from_event_loop(move || {
                    let lcd = Image::from_rgba8_premultiplied(buffer);
                    //ui_handle.upgrade().unwrap().set_lcd(lcd);

                    ui_handle.upgrade().unwrap().global::<Lcd>().set_lcd(lcd);

                    while let Some(event) = ui_events.pop() {
                        outputs.set_state(&ui_handle, event);
                    }
                }).unwrap();

                let ui_handle = ui_handle_.clone();
                let log_info = logger.to_display();
                slint::invoke_from_event_loop(move || {
                    ui_handle.upgrade().unwrap().global::<LogSettings>().set_logger(log_info.into());
                }).unwrap();
            }
        }
    }
}
