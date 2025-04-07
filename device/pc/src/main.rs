mod dev_const;
mod display;
mod eeprom;
mod tcp;

use byteorder::{ByteOrder, LittleEndian as LE};
use corelib::*;
use dev_const::{DISPLAY_HEIGHT, DISPLAY_WIDTH};

use display::MockDisplay;
use eeprom::Storage;
use embedded_graphics::prelude::*;
use embedded_graphics_simulator::{sdl2::Keycode, OutputSettings, SimulatorEvent, Window};
use heapless::spsc::Queue;
use std::{
    net::UdpSocket,
    process,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use tcp::TcpServer;

fn millis() -> u16 {
    let since_the_epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    since_the_epoch.as_millis() as u16
}

const SW_VERSION: SwVersion = SwVersion {
    version: [0, 1, 1, 0],
};
const HW_VERSION: HwVersion = HwVersion {
    version: [1, 3, 1, 0],
};

fn main() -> Result<(), core::convert::Infallible> {
    println!(
        "Use the following keys for operation:\n\n\
    F1 Button 1\n\
    F2 Button 2\n\
    F3 Button 3\n\
    F4 Button 4\n\
\
    F5 Button Encoder\n
    F5 Button Encoder 3 seconds\n\n\
\
    ⇒ Small Encoder right\n\
    ⇐ Small Encoder left\n\
    ⇑  Big Encoder right\n\
    ⇓  Big Encoder left\n\n\
\
    1..4 Toggle Input 1..4\n
\
    S Key to save image as png file\n\
    U Key to simulate Firmware Update\n\n\
\
    <ENTER> Button Encoder\n\
"
    );

    let display = MockDisplay::new(Size::new(DISPLAY_WIDTH, DISPLAY_HEIGHT));
    let mut window = Window::new("Vario - Mock", &OutputSettings::default());

    let (p_idle_events, mut c_idle_events) = spsc_queue!(QIdleEvents);
    let (p_tx_frames, mut c_tx_frames) = spsc_queue!(QTxFrames<10>);

    let mut core_model = CoreModel::new(&dev_const::DEVICE_CONST, 0x1234_5678);

    let mut eeprom = Storage::new().unwrap();
    let mut nmea_server = TcpServer::new("127.0.0.1:4353");

    let mut in1 = PinState::High;
    let mut in2 = PinState::High;
    let mut in3 = PinState::High;
    let mut in4 = PinState::High;

    let mut controller = CoreController::new(&mut core_model, p_idle_events, p_tx_frames);
    for item in eeprom.iter_over(EepromTopic::ConfigValues) {
        persist::restore_item(&mut controller, &mut core_model, item);
        // println!("Restored {:?}", item);
    }

    let mut view = CoreView::new(display, &core_model);
    let socket = UdpSocket::bind("127.0.0.1:5005").expect("Could not open UDP socket");
    socket
        .set_read_timeout(Some(Duration::from_millis(10)))
        .expect("Could not set read timeout");

    let mut img_no = 0_u32;
    let mut sw_update_status = 0_u32;
    controller.set_ms(millis());
    controller.recalc_glider(&mut core_model);

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

                        Keycode::F5 | Keycode::Return => KeyEvent::BtnEnc,
                        Keycode::F6 => KeyEvent::BtnEncS3,

                        Keycode::Num1 => {
                            in1 = !in1;
                            println!("Input1 {:?}", in1);
                            let event = Event::InputItem(InputPinState::Io1(in1));
                            controller.event_handler(event, &mut core_model);
                            KeyEvent::NoEvent
                        }
                        Keycode::Num2 => {
                            in2 = !in2;
                            println!("Input1 {:?}", in2);
                            let event = Event::InputItem(InputPinState::Io2(in2));
                            controller.event_handler(event, &mut core_model);
                            KeyEvent::NoEvent
                        }
                        Keycode::Num3 => {
                            in3 = !in3;
                            KeyEvent::NoEvent
                        }
                        Keycode::Num4 => {
                            in4 = !in4;
                            KeyEvent::NoEvent
                        }

                        Keycode::C => break 'running,
                        Keycode::S => {
                            img_no += 1;
                            let img_path = format!("vario_{:03}.png", img_no);
                            view.display.save_png(&img_path);
                            if DISPLAY_WIDTH == 480 && DISPLAY_HEIGHT == 480 {
                                let img_wh_path = format!("vario_wh_{:03}.png", img_no);
                                view.display.save_png_with_housing(&img_wh_path);
                                println!(
                                    "Image '{}' and '{}' saved to disk",
                                    &img_path, &img_wh_path
                                );
                            } else {
                                println!("Image '{}' saved to disk", &img_path);
                            }
                            KeyEvent::NoEvent
                        }
                        Keycode::U => {
                            let device_event = match sw_update_status {
                                0 => {
                                    sw_update_status = 1;
                                    DeviceEvent::FwAvailable(
                                        core_model.device_const.misc.sw_version,
                                    )
                                }
                                _ => {
                                    sw_update_status = 0;
                                    DeviceEvent::UploadFinished
                                }
                            };
                            let event = Event::DeviceItem(device_event);
                            controller.event_handler(event, &mut core_model);
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
            let event = Event::KeyItem(key_event);
            controller.event_handler(event, &mut core_model);
        }
        controller.tick_1ms(millis(), &mut core_model);
        view.prepare(&core_model);
        view.draw().unwrap();

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
                IdleEvent::DateTime(_) | IdleEvent::SetGain(_) => (),
                _ => (), // println!("IdleEvent {:?}", &idle_event),
            }
            match idle_event {
                IdleEvent::SetEepromItem(item) => {
                    eeprom.write_item(item).unwrap();
                }
                IdleEvent::ClearEepromItems(items_list) => {
                    eeprom.delete_items_list(items_list).unwrap();
                }
                IdleEvent::SdCardItem(item) => {
                    if item == SdCardCmd::SwUpdateCanceled {
                        sw_update_status = 0
                    }
                }
                IdleEvent::FeedTheDog => (), // No Watchdog in this demo app
                IdleEvent::SetGain(_) => (), // Sound is done via can datagram
                IdleEvent::DateTime(_) => (), // Date and time for crash reports are not required
                IdleEvent::ResetDevice(_reason) => {
                    println!("Device reset - please restart!");
                    process::exit(1);
                }
                IdleEvent::Output1(_) | IdleEvent::Output2(_) => (),
            }
        }

        while let Some(nmea_data) = controller.nmea_next(&mut core_model) {
            nmea_server.send(nmea_data);
            if nmea_data.len() >= 6 && &nmea_data[0..6] == b"$PLARS" {
                print!("{}", std::str::from_utf8(nmea_data).unwrap());
            }
        }

        let mut buf = [0u8; 10];
        while let Ok((cnt, _adr)) = socket.recv_from(&mut buf) {
            let id = LE::read_u16(&buf[..2]);
            let can_frame = CanFrame::from_slice(id, &buf[2..cnt]);
            let frame = if id >= 0x120 && id <= 0x12f {
                Frame::Specific(SpecificFrame {
                    can_frame,
                    specific_id: id & 0x0f,
                    object_id: 2, // Sensorbox
                })
            } else if id >= 0x140 && id <= 0x14f {
                Frame::Specific(SpecificFrame {
                    can_frame,
                    specific_id: id & 0x0f,
                    object_id: 3, // GPS
                })
            } else {
                Frame::Legacy(can_frame)
            };
            controller.read_can_frame(&mut core_model, &frame);
        }

        if let Some(rx_data) = nmea_server.recv() {
            controller.nmea_recv_slice(&mut core_model, rx_data.as_slice());
            print!("{}", std::str::from_utf8(&rx_data).unwrap());
        }
    }
    Ok(())
}
