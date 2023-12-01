use stm32f4xx_hal::gpio::Pin;

const BTN_1: u8 = 0b0000_0001;
const BTN_2: u8 = 0b0000_0010;
const BTN_3: u8 = 0b0000_0100;
const BTN_ESC: u8 = 0b0000_1000;
const BTN_ENC: u8 = 0b0001_0000;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Key {
    Button1,
    Button2,
    Button3,
    Button4,
    ButtonKnob,
    None,
}

pub struct Keyboard {
    btn_1: Pin<'A', 7>,
    btn_2: Pin<'C', 5>,
    btn_3: Pin<'B', 0>,
    btn_esc: Pin<'B', 1>,
    btn_enc: Pin<'A', 6>,

    last_btn_state: u8,
    first_go_to_0: bool,
}

pub struct KeyboardPins {
    pub p1: Pin<'A', 7>,
    pub p2: Pin<'C', 5>,
    pub p3: Pin<'B', 0>,
    pub p4: Pin<'B', 1>,
    pub p5: Pin<'A', 6>,
}

impl KeyboardPins {
    pub fn new(
        p1: Pin<'A', 7>,
        p2: Pin<'C', 5>,
        p3: Pin<'B', 0>,
        p4: Pin<'B', 1>,
        p5: Pin<'A', 6>,
    ) -> Self {
        KeyboardPins { p1, p2, p3, p4, p5 }
    }
}

impl Keyboard {
    /// Create Keys instance and initialize hardware
    pub fn new(keyboard_pins: KeyboardPins) -> Self {
        // Config the key buttons
        let btn_1 = keyboard_pins.p1.into_pull_up_input();
        let btn_2 = keyboard_pins.p2.into_pull_up_input();
        let btn_3 = keyboard_pins.p3.into_pull_up_input();
        let btn_esc = keyboard_pins.p4.into_pull_up_input();
        let btn_enc = keyboard_pins.p5.into_pull_up_input();

        Self {
            btn_1, // 5 port pins for buttons
            btn_2,
            btn_3,
            btn_esc,
            btn_enc,

            last_btn_state: 0,    // last state of buttons
            first_go_to_0: false, // we have to wait, befor we accept new key events
        }
    }

    /// Regular polling of the front keys and the rotary encoders
    ///
    /// This routine is called about every 20 milliseconds and queries the 5 keys and the two
    /// rotary encoders. The result is pushed into the key event queue.
    pub fn tick(&mut self) -> Key {
        // First of all, the pushbuttons are scanned. The events are summarized in a status
        // variable in order to evaluate them afterwards. This allows single buttons to be
        // used in the same way as button patterns.
        let mut btn_state = 0u8;

        if self.btn_1.is_low() {
            btn_state |= BTN_1
        }
        if self.btn_2.is_low() {
            btn_state |= BTN_2
        }
        if self.btn_3.is_low() {
            btn_state |= BTN_3
        }
        if self.btn_esc.is_low() {
            btn_state |= BTN_ESC
        }
        if self.btn_enc.is_low() {
            btn_state |= BTN_ENC
        }

        if self.first_go_to_0 {
            if btn_state == 0 {
                self.first_go_to_0 = false;
            }
        } else if btn_state < self.last_btn_state {
            // Triggers when first key is released
            self.first_go_to_0 = true;
            match self.last_btn_state {
                BTN_1 => return Key::Button1,
                BTN_2 => return Key::Button2,
                BTN_3 => return Key::Button3,
                BTN_ESC => return Key::Button4,
                BTN_ENC => return Key::ButtonKnob,
                _ => (),
            };
        }
        self.last_btn_state = btn_state;
        Key::None
    }
}
