use crate::driver::QEvents;
use corelib::{Event, InputPinState, KeyEvent, PinState};
use stm32h7xx_hal::{
    gpio::{Input, Pin},
    pac::{TIM3, TIM5},
    rcc::{
        rec::{Tim3, Tim5},
        ResetEnable,
    },
};

const BTN_1: u8 = 0b0000_0001;
const BTN_2: u8 = 0b0000_0010;
const BTN_ENC: u8 = 0b0001_0000;
const BTN_1_2: u8 = 0b0000_0011;

const ENC_1_START_VALUE: u32 = 0x8000;
const ENC_2_START_VALUE: u16 = 0x8000;

pub struct Keyboard {
    kbd_pins: KeyboardPins,
    input_pins: InputPins,

    tim_enc_1: TIM5,
    enc_1_cnt: u32,
    tim_enc_2: TIM3,
    enc_2_cnt: u16,

    last_btn_state: u8,
    first_go_to_0: bool,
    tick_cnt: u8,

    io1_state: PinState,
    io2_state: PinState,
    io3_state: PinState,
    io4_state: PinState,
    io_tick: u8,

    q_events: &'static QEvents,
}

pub struct KeyboardPins {
    pub btn1: Pin<'E', 5, Input>,
    pub btn2: Pin<'E', 6, Input>,
    pub btn_enc: Pin<'E', 4, Input>,
}

impl KeyboardPins {
    pub fn new(btn1: Pin<'E', 5>, btn2: Pin<'E', 6>, btn_enc: Pin<'E', 4>) -> Self {
        KeyboardPins {
            btn1: btn1.into_input(),
            btn2: btn2.into_input(),
            btn_enc: btn_enc.into_input(),
        }
    }
}

pub struct Enc1Res {
    tim_p_1: Tim5,
    tim_enc_1: TIM5,
    enc_1a: Pin<'A', 0>,
    enc_1b: Pin<'A', 1>,
}

impl Enc1Res {
    pub fn new(tim_p_1: Tim5, tim_enc_1: TIM5, enc_1a: Pin<'A', 0>, enc_1b: Pin<'A', 1>) -> Self {
        Enc1Res {
            tim_p_1,
            tim_enc_1,
            enc_1a,
            enc_1b,
        }
    }
}

pub struct Enc2Res {
    tim_p_2: Tim3,
    tim_enc_2: TIM3,
    enc_2a: Pin<'B', 4>,
    enc_2b: Pin<'A', 7>,
}

impl Enc2Res {
    pub fn new(tim_p_2: Tim3, tim_enc_2: TIM3, enc_2a: Pin<'B', 4>, enc_2b: Pin<'A', 7>) -> Self {
        Enc2Res {
            tim_p_2,
            tim_enc_2,
            enc_2a,
            enc_2b,
        }
    }
}

pub struct InputPins {
    di1: Pin<'B', 12, Input>,
    di2: Pin<'B', 13, Input>,
    di3: Pin<'D', 12, Input>,
    di4: Pin<'D', 13, Input>,
}

impl InputPins {
    pub fn new(
        di1: Pin<'B', 12>,
        di2: Pin<'B', 13>,
        di3: Pin<'D', 12>,
        di4: Pin<'D', 13>,
    ) -> Self {
        InputPins {
            di1: di1.into_input(),
            di2: di2.into_input(),
            di3: di3.into_input(),
            di4: di4.into_input(),
        }
    }
}

impl Keyboard {
    /// Create Keys instance and initialize hardware
    pub fn new(
        keyboard_pins: KeyboardPins,
        input_pins: InputPins,
        enc1_res: Enc1Res,
        enc2_res: Enc2Res,
        q_events: &'static QEvents,
    ) -> Self {
        // Config encoder 1 port pins
        let _ = enc1_res.enc_1a.into_alternate::<2>(); // Set to alternate function 2
        let _ = enc1_res.enc_1b.into_alternate::<2>(); // Set to alternate function 2

        // Encoder 1
        // Timer 5 ch 1 -> PA0 (Encoder 1A)
        // Timer 5 ch 2 -> PA1 (Encoder 1B)
        enc1_res.tim_p_1.enable().reset();
        let tim_enc_1 = enc1_res.tim_enc_1;
        tim_enc_1.cnt.write(|w| w.cnt().bits(ENC_1_START_VALUE));
        tim_enc_1.smcr.write(|w| w.sms().bits(1)); // Encoder mode 1
        tim_enc_1
            .ccmr1_input()
            .write(|w| unsafe { w.cc1s().bits(1) }); // CC1 is input IC1 -> TI1
        tim_enc_1
            .ccmr1_input()
            .write(|w| unsafe { w.cc2s().bits(1) }); // CC2 is input IC2 -> TI2
        tim_enc_1
            .cr1
            .modify(|_, w| w.cen().set_bit().udis().set_bit()); // Enable timer 4
        let enc_1_cnt = tim_enc_1.cnt.read().cnt().bits();

        // Config encoder 2 port pins
        let _ = enc2_res.enc_2a.into_alternate::<2>(); // Set to alternate function 2
        let _ = enc2_res.enc_2b.into_alternate::<2>(); // Set to alternate function 2

        // Encoder 2
        // Timer 3 ch 1 -> PB4 (Encoder 2B)
        // Timer 3 ch 2 -> PA7 (Encoder 2A)
        enc2_res.tim_p_2.enable().reset();
        let tim_enc_2 = enc2_res.tim_enc_2;
        tim_enc_2.cnt.write(|w| w.cnt().bits(ENC_2_START_VALUE));
        tim_enc_2.smcr.write(|w| w.sms().bits(1)); // Encoder mode 1
        tim_enc_2
            .ccmr1_input()
            .write(|w| unsafe { w.cc1s().bits(1) }); // CC1 is input IC1 -> TI1
        tim_enc_2
            .ccmr1_input()
            .write(|w| unsafe { w.cc2s().bits(1) }); // CC2 is input IC2 -> TI2
        tim_enc_2
            .cr1
            .modify(|_, w| w.cen().set_bit().udis().set_bit()); // Enable timer 5
        let enc_2_cnt = tim_enc_2.cnt.read().cnt().bits();

        let io1_state = PinState::from(input_pins.di1.is_high());
        let _ = q_events.enqueue(Event::InputItem(InputPinState::Io1(io1_state)));
        let io2_state = PinState::from(input_pins.di2.is_high());
        let _ = q_events.enqueue(Event::InputItem(InputPinState::Io2(io2_state)));
        let io3_state = PinState::from(input_pins.di3.is_high());
        let _ = q_events.enqueue(Event::InputItem(InputPinState::Io3(io3_state)));
        let io4_state = PinState::from(input_pins.di4.is_high());
        let _ = q_events.enqueue(Event::InputItem(InputPinState::Io4(io4_state)));

        Self {
            kbd_pins: keyboard_pins,
            input_pins,
            tim_enc_1, // 2 timer for rotary encoder
            enc_1_cnt,
            tim_enc_2,
            enc_2_cnt,

            last_btn_state: 0,      // last state of buttons
            first_go_to_0: false,   // we have to wait, befor we accept new key events
            tick_cnt: 0,            // tick counter for timing functionality

            io1_state,
            io2_state,
            io3_state,
            io4_state,
            io_tick: 0,             // 

            q_events, // queue to push key events
        }
    }

    /// Regular polling of the front keys and the rotary encoders (50 Hz)
    pub fn tick(&mut self) {
        self.keyboard_tick();
        self.io_tick();
    }

    /// This routine is called at 50 Hz
    fn io_tick(&mut self) {
        self.io_tick += 1;
        if self.io_tick < 5 {
            return;
        }
        // every 100ms
        self.io_tick = 0;
        let state = PinState::from(self.input_pins.di1.is_high());
        if state != self.io1_state {
            self.io1_state = state;
            let _ = self.q_events.enqueue(Event::InputItem(InputPinState::Io1(self.io1_state)));
        }
        let state = PinState::from(self.input_pins.di2.is_high());
        if state != self.io2_state {
            self.io2_state = state;
            let _ = self.q_events.enqueue(Event::InputItem(InputPinState::Io2(self.io2_state)));
        }
        let state = PinState::from(self.input_pins.di3.is_high());
        if state != self.io3_state {
            self.io3_state = state;
            let _ = self.q_events.enqueue(Event::InputItem(InputPinState::Io3(self.io3_state)));
        }
        let state = PinState::from(self.input_pins.di4.is_high());
        if state != self.io4_state {
            self.io4_state = state;
            let _ = self.q_events.enqueue(Event::InputItem(InputPinState::Io4(self.io4_state)));
        }
    }

    /// This routine is called about every 20 milliseconds and queries the 5 keys and the two
    /// rotary encoders. The result is pushed into the key event queue.
    fn keyboard_tick(&mut self) {
        // First of all, the pushbuttons are scanned. The events are summarized in a status
        // variable in order to evaluate them afterwards. This allows single buttons to be
        // used in the same way as button patterns.
        let mut btn_state = 0u8;

        if self.kbd_pins.btn_enc.is_low() {
            btn_state |= BTN_ENC
        }

        if self.first_go_to_0 {
            if btn_state == 0 {
                self.first_go_to_0 = false;
                self.tick_cnt = 0;
            }
        } else if btn_state < self.last_btn_state {
            // Triggers when first key is released
            let _ = match self.last_btn_state {
                BTN_1 => self.q_events.enqueue(Event::KeyItem(KeyEvent::Btn1)),
                BTN_2 => self.q_events.enqueue(Event::KeyItem(KeyEvent::Btn2)),
                BTN_ENC => self.q_events.enqueue(Event::KeyItem(KeyEvent::BtnEnc)),
                BTN_1_2 => self.q_events.enqueue(Event::KeyItem(KeyEvent::Btn12)),
                _ => Ok(()),
            };
            self.first_go_to_0 = true;
            self.tick_cnt = 0;
        } else if btn_state > 0 {
            self.tick_cnt = self.tick_cnt.saturating_add(1);
            // Triggers when keys are pressed for more then 3 seconds
            if self.tick_cnt > 60 {
                let _ = match btn_state {
                    BTN_1 => self.q_events.enqueue(Event::KeyItem(KeyEvent::Btn1S3)),
                    BTN_2 => self.q_events.enqueue(Event::KeyItem(KeyEvent::Btn2S3)),
                    BTN_ENC => self.q_events.enqueue(Event::KeyItem(KeyEvent::BtnEncS3)),
                    BTN_1_2 => self.q_events.enqueue(Event::KeyItem(KeyEvent::Btn12S3)),
                    _ => Ok(()),
                };
                self.first_go_to_0 = true;
            }
        }
        self.last_btn_state = btn_state;

        // Interrogation and evaluation of the rotary encoders
        let count = self.tim_enc_1.cnt.read().cnt().bits();

        if count != self.enc_1_cnt {
            let mut delta = count.wrapping_sub(self.enc_1_cnt) as i16;
            while delta > 0 {
                let _ = self
                    .q_events
                    .enqueue(Event::KeyItem(KeyEvent::Rotary2Right));
                delta -= 1;
            }
            while delta < 0 {
                let _ = self.q_events.enqueue(Event::KeyItem(KeyEvent::Rotary2Left));
                delta += 1;
            }
            self.enc_1_cnt = count;
        }

        let count = self.tim_enc_2.cnt.read().cnt().bits();

        if count != self.enc_2_cnt {
            let mut delta = count.wrapping_sub(self.enc_2_cnt) as i16;
            while delta > 0 {
                let _ = self
                    .q_events
                    .enqueue(Event::KeyItem(KeyEvent::Rotary1Right));
                delta -= 1;
            }
            while delta < 0 {
                let _ = self.q_events.enqueue(Event::KeyItem(KeyEvent::Rotary1Left));
                delta += 1;
            }
            self.enc_2_cnt = count;
        }
    }
}
