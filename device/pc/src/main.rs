mod display;
mod eeprom;
mod tcp;

use byteorder::{ByteOrder, LittleEndian as LE};
use corelib::{
    basic_config::{DISPLAY_HEIGHT, DISPLAY_WIDTH},
    *,
};

const SW_VERSION: SwVersion = SwVersion {
    version: [0, 1, 1, 0],
};
const HW_VERSION: HwVersion = HwVersion {
    version: [1, 3, 1, 0],
};

use display::MockDisplay;
use eeprom::Storage;
use embedded_graphics::prelude::*;
use embedded_graphics_simulator::{sdl2::Keycode, OutputSettings, SimulatorEvent, Window};
use heapless::spsc::Queue;
use std::{net::UdpSocket, time::Duration};
use tcp::TcpServer;

fn main() -> Result<(), core::convert::Infallible> {
    println!(
        "Use the following keys for operation:\n\n\
    F1 Button 1\n\
    F2 Button 2\n\
    F3 Button 3\n\
    F4 Button Escape\n\
    F5 Button Encoder\n\n\
\
    ⇒ Small Encoder right\n\
    ⇐ Small Encoder left\n\
    ⇑  Big Encoder right\n\
    ⇓  Big Encoder left\n\n\
\
    F8 Button 1 and Esc fro 3 secs (Domo Mode)\n\
    F9 Button 1 for 3 secs (Glider)\n\
    F10 Button 2 for 3 secs (Dark/Bright Mode)\n\n\
\
    S Key to save image as png file\n\
    U Key to simulate Firmware Update\n\
    N Key to export available NMEA strings\n\
"
    );

    let display = MockDisplay::new(Size::new(DISPLAY_WIDTH, DISPLAY_HEIGHT));
    let mut window = Window::new("Vario - Mock", &OutputSettings::default());

    // This queue routes the PersItems from the controller to the idle loop.
    let (p_idle_events, mut c_idle_events) = {
        static mut Q_IDLE_EVENTS: QIdleEvents = Queue::new();
        // Note: unsafe is ok here, because [heapless::spsc] queue protects against UB
        unsafe { Q_IDLE_EVENTS.split() }
    };
    // This queue transports the can bus frames from the view component to the can tx driver.
    let (p_tx_frames, mut c_tx_frames) = {
        static mut Q_TX_FRAMES: QTxFrames<10> = Queue::new();
        // Note: unsafe is ok here, because [heapless::spsc] queue protects against UB
        unsafe { Q_TX_FRAMES.split() }
    };

    let mut core_model = CoreModel::new(
        p_idle_events,
        p_tx_frames,
        0x1234_5678,
        HW_VERSION,
        SW_VERSION,
    );
    let mut eeprom = Storage::new().unwrap();
    let mut nmea_server = TcpServer::new("127.0.0.1:4353");

    for item in eeprom.iter_over(EepromTopic::ConfigValues) {
        core_model.restore_persistent_item(item);
        println!("Restored {:?}", item);
    }

    let mut controller = CoreController::new(&mut core_model);
    let mut view = CoreView::new(display);
    let socket = UdpSocket::bind("127.0.0.1:5005").expect("Could not open UDP socket");
    socket
        .set_read_timeout(Some(Duration::from_millis(40)))
        .expect("Could not set read timeout");

    let mut img_no = 0_u32;
    let mut sw_update_status = 0_u32;

    'running: loop {
        window.update(&view.display.display);

        let mut key_event = KeyEvent::NoEvent;
        for event in window.events() {
            match event {
                SimulatorEvent::Quit => break 'running,
                SimulatorEvent::KeyDown { keycode, .. } => {
                    key_event = match keycode {
                        Keycode::Down => KeyEvent::Rotary1Left,
                        Keycode::Up => KeyEvent::Rotary1Right,
                        Keycode::Left => KeyEvent::Rotary2Left,
                        Keycode::Right => KeyEvent::Rotary2Right,
                        Keycode::F1 => KeyEvent::Btn1,
                        Keycode::F2 => KeyEvent::Btn2,
                        Keycode::F3 => KeyEvent::Btn3,
                        Keycode::F4 => KeyEvent::BtnEsc,
                        Keycode::F5 => KeyEvent::BtnEnc,
                        Keycode::F8 => KeyEvent::Btn1EscS3,
                        Keycode::F9 => KeyEvent::Btn1S3,
                        Keycode::F10 => KeyEvent::Btn2S3,

                        Keycode::S => {
                            img_no += 1;
                            let img_path = format!("vario_{:03}.png", img_no);
                            println!("Image {} saved to disk", &img_path);
                            view.display.save_png(&img_path);
                            KeyEvent::NoEvent
                        }
                        Keycode::U => {
                            let device_event = match sw_update_status {
                                0 => DeviceEvent::FwAvailable(SW_VERSION),
                                1 => DeviceEvent::PrepareFwUpload,
                                2 => DeviceEvent::UploadInProgress,
                                3 => DeviceEvent::UploadFinished,
                                _ => DeviceEvent::UploadFinished,
                            };
                            sw_update_status = if sw_update_status == 3 {
                                0
                            } else {
                                sw_update_status + 1
                            };
                            controller.device_action(&mut core_model, &device_event);
                            KeyEvent::NoEvent
                        }
                        Keycode::Kp1 => {
                            core_model.device.supply_voltage = 13.0;
                            KeyEvent::NoEvent
                        }
                        Keycode::Kp2 => {
                            core_model.device.supply_voltage = 11.0;
                            KeyEvent::NoEvent
                        }
                        Keycode::Kp3 => {
                            core_model.device.supply_voltage = 10.0;
                            KeyEvent::NoEvent
                        }
                        _ => {
                            println!("Key with no effect {:?}", keycode);
                            KeyEvent::NoEvent
                        }
                    };
                }
                _ => {}
            }
        }
        if key_event != KeyEvent::NoEvent {
            controller.key_action(&mut core_model, &key_event);
        }
        controller.time_action(&mut core_model);
        view.draw(&mut core_model).unwrap();

        while c_tx_frames.len() > 0 {
            let frame = c_tx_frames.dequeue().unwrap();
            if let Frame::Specific(specific_frame) = frame {
                if specific_frame.specific_id == 0 {
                    let data = specific_frame.can_frame.data();
                    let frequency = LE::read_u16(&data[..2]);
                    let duty_cycle = LE::read_u16(&data[2..4]);
                    let volume = data[4];
                    let continuous = data[5] == 1;
                    window.sound(frequency, volume, continuous, duty_cycle);
                }
            }
        }

        while c_idle_events.len() > 0 {
            let idle_event = c_idle_events.dequeue().unwrap();
            match idle_event {
                IdleEvent::DateTime(_) => (),
                _ => println!("IdleEvent {:?}", &idle_event),
            }
            match idle_event {
                IdleEvent::EepromItem(item) => {
                    eeprom.write_item(item).unwrap();
                    if let Some(nmea_str) = core_model.nmea_plars(item.id) {
                        nmea_server.send(nmea_str);
                    } 
                }
                IdleEvent::SdCardItem(item) => {
                    if item == SdCardCmd::SwUpdateCanceled {
                        sw_update_status = 0
                    }
                }
                IdleEvent::FeedTheDog => (), // No Watchdog in this demo app
                IdleEvent::SetGain(_) => (), // Sound is done via can datagram
                IdleEvent::DateTime(_) => (), // Date and time for crash reports are not required
            }
        }

        while let Some(nmea_data) = core_model.nmea_next() {
            nmea_server.send(nmea_data);
        }

        let mut buf = [0u8; 10];
        while let Ok((cnt, _adr)) = socket.recv_from(&mut buf) {
            let id = LE::read_u16(&buf[..2]);
            let can_frame = CanFrame::from_slice(id, &buf[2..cnt]);
            let frame = Frame::Legacy(can_frame);
            controller.read_can_frame(&mut core_model, &frame);
        }

        if let Some(rx_data) = nmea_server.recv() {
            core_model.nmea_recv_slice(rx_data.as_slice());
        }
    }
    Ok(())
}
