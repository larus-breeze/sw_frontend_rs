use embedded_graphics::prelude::*;
use embedded_graphics_simulator::{
    sdl2::Keycode, OutputSettings, SimulatorDisplay, SimulatorEvent, Window,
};
use std::{net::UdpSocket, time::Duration};
use byteorder::{ByteOrder, LittleEndian as LE};

use vario_display::*;


struct MockDisplay {
    pub display: SimulatorDisplay<Colors>,
}


impl MockDisplay {
    /// Creates a new display.
    ///
    /// The display is filled with `C::from(BinaryColor::Off)`.
    pub fn new(size: Size) -> Self {
        let display = SimulatorDisplay::with_default_color(size, Colors::Black);
        MockDisplay{display}
    }
}

impl DrawImage for MockDisplay {
    fn draw_img(&mut self, img: &[u8], offset: Point) -> Result<(), CoreError> {
        // Safety: the img format has been defined in terms of compatibility, so the conversion is ok here
        let img16 =
            unsafe { core::slice::from_raw_parts(img.as_ptr() as *const u16, img.len() / 2) };
        // At the moment we only know format 1
        assert!(img16[0] == 1);

        // The image is really built for our display?
        assert!(img16[1] == DISPLAY_WIDTH as u16);
        assert!(img16[2] + offset.y as u16 <= DISPLAY_HEIGHT as u16);

        // Let's write the pixels
        let color_cnt = img16[3];
        let mut idx = 4;
        for _ in 0..color_cnt {
            let color = Colors::from(img16[idx] as u8);
            let px_cnt = img16[idx + 1] as usize;
            idx += 2;
            for idx in idx..idx + px_cnt {
                let i_idx = img16[idx];
                let y = i_idx / (DISPLAY_WIDTH as u16);
                let x = i_idx as u16 - y * DISPLAY_WIDTH as u16;
                let p = Point::new(offset.x + x as i32, offset.y + y as i32);
                let _ = Pixel(p, color).draw(self);
            }
            idx += px_cnt;
        }
        Ok(())
    }
}

impl DrawTarget for MockDisplay {
    type Color = Colors;
    type Error = CoreError;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        self.display.draw_iter(pixels).unwrap();
        Ok(())
    }
}

impl OriginDimensions for MockDisplay {
    fn size(&self) -> Size {
        self.display.size()
    }
}

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
    F9 Button 1 for 3 secs (Glider)\n
"
    );

    let display = MockDisplay::new(Size::new(DISPLAY_WIDTH, DISPLAY_HEIGHT));
    let mut window = Window::new("Vario - Mock", &OutputSettings::default());

    let mut core_model = CoreModel::default();
    let mut controller = CoreController::new(&mut core_model);
    let mut view = CoreView::new(display);
    let socket = UdpSocket::bind("127.0.0.1:5005").expect("Could not open UDP socket");
    socket.set_read_timeout(Some(Duration::from_millis(40))).expect("Could not set read timeout");

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

        let mut buf = [0u8; 10];
        loop {
            match socket.recv_from(&mut buf) {
                Ok((cnt, _adr)) => {
                    let id = LE::read_u16(&buf[..2]);
                    let frame = CanFrame::from_slice(id, &buf[2..cnt]);
                    controller.read_can_frame(&mut core_model, &frame);
                }
                Err(_) => break,
            }
        }
    }
    Ok(())
}
