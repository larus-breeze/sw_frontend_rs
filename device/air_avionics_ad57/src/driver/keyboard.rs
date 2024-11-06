use crate::driver::QEvents;
use corelib::{Event, KeyEvent};
use stm32f4xx_hal::{
    gpio::Pin,
    pac::{RCC, TIM4, TIM5},
};

const BTN_1: u8 = 0b0000_0001;
const BTN_2: u8 = 0b0000_0010;
const BTN_3: u8 = 0b0000_0100;
const BTN_ESC: u8 = 0b0000_1000;
const BTN_ENC: u8 = 0b0001_0000;
const BTN_1_2: u8 = 0b0000_0011;
const BTN_2_3: u8 = 0b0000_0110;
const BTN_3_ESC: u8 = 0b0000_1100;
const BTN_1_ESC: u8 = 0b0000_1001;
const BTN_1_ESC_ENC: u8 = 0b0001_1001;

const ENC_1_START_VALUE: u16 = 0x8000;
const ENC_2_START_VALUE: u32 = 0x8000;

pub struct Keyboard {
    btn_1: Pin<'A', 7>,
    btn_2: Pin<'C', 5>,
    btn_3: Pin<'B', 0>,
    btn_esc: Pin<'B', 1>,
    btn_enc: Pin<'A', 6>,

    tim_enc_1: TIM4,
    enc_1_cnt: u16,
    tim_enc_2: TIM5,
    enc_2_cnt: u32,

    last_btn_state: u8,
    first_go_to_0: bool,
    tick_cnt: u32,
    q_events: &'static QEvents,
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

pub struct Enc1Res {
    tim_enc_1: TIM4,
    enc_1a: Pin<'D', 12>,
    enc_1b: Pin<'D', 13>,
}

impl Enc1Res {
    pub fn new(tim_enc_1: TIM4, enc_1a: Pin<'D', 12>, enc_1b: Pin<'D', 13>) -> Self {
        Enc1Res {
            tim_enc_1,
            enc_1a,
            enc_1b,
        }
    }
}

pub struct Enc2Res {
    tim_enc_2: TIM5,
    enc_2a: Pin<'A', 0>,
    enc_2b: Pin<'A', 1>,
}

impl Enc2Res {
    pub fn new(tim_enc_2: TIM5, enc_2a: Pin<'A', 0>, enc_2b: Pin<'A', 1>) -> Self {
        Enc2Res {
            tim_enc_2,
            enc_2a,
            enc_2b,
        }
    }
}

impl Keyboard {
    /// Create Keys instance and initialize hardware
    pub fn new(
        keyboard_pins: KeyboardPins,
        enc1_res: Enc1Res,
        enc2_res: Enc2Res,
        q_events: &'static QEvents,
    ) -> Self {
        // Config the key buttons
        let btn_1 = keyboard_pins.p1.into_pull_up_input();
        let btn_2 = keyboard_pins.p2.into_pull_up_input();
        let btn_3 = keyboard_pins.p3.into_pull_up_input();
        let btn_esc = keyboard_pins.p4.into_pull_up_input();
        let btn_enc = keyboard_pins.p5.into_pull_up_input();

        // Config encoder 1 port pins
        let _ = enc1_res.enc_1a.into_alternate::<2>(); // Set to alternate function 2
        let _ = enc1_res.enc_1b.into_alternate::<2>(); // Set to alternate function 2

        //NOTE(unsafe) this reference will only be used for atomic writes with no side effects
        let rcc = unsafe { &(*RCC::ptr()) };

        // Config encoder 1 timer
        // Safety (unsafe) atomic writes are ok here
        let tim_enc_1 = enc1_res.tim_enc_1;
        rcc.apb1enr.modify(|_, w| w.tim4en().set_bit()); // Enable timer 4
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

        // Config encoder 2 timer
        // Safety (unsafe) atomic writes are ok here
        let tim_enc_2 = enc2_res.tim_enc_2;
        rcc.apb1enr.modify(|_, w| w.tim5en().set_bit()); // Enable timer 5
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

        Self {
            btn_1, // 5 port pins for buttons
            btn_2,
            btn_3,
            btn_esc,
            btn_enc,

            tim_enc_1, // 2 timer for rotary encoder
            enc_1_cnt,
            tim_enc_2,
            enc_2_cnt,

            last_btn_state: 0,    // last state of buttons
            first_go_to_0: false, // we have to wait, befor we accept new key events
            tick_cnt: 0,          // tick counter for timing functionality

            q_events, // queue to push key events
        }
    }

    /// Regular polling of the front keys and the rotary encoders
    ///
    /// This routine is called about every 20 milliseconds and queries the 5 keys and the two
    /// rotary encoders. The result is pushed into the key event queue.
    pub fn tick(&mut self) {
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
                self.tick_cnt = 0;
            }
        } else if btn_state < self.last_btn_state {
            // Triggers when first key is released
            let _ = match self.last_btn_state {
                BTN_1 => self.q_events.enqueue(Event::KeyItem(KeyEvent::Btn1)),
                BTN_2 => self.q_events.enqueue(Event::KeyItem(KeyEvent::Btn2)),
                BTN_3 => self.q_events.enqueue(Event::KeyItem(KeyEvent::Btn3)),
                BTN_ESC => self.q_events.enqueue(Event::KeyItem(KeyEvent::BtnEsc)),
                BTN_ENC => self.q_events.enqueue(Event::KeyItem(KeyEvent::BtnEnc)),
                BTN_1_2 => self.q_events.enqueue(Event::KeyItem(KeyEvent::Btn12)),
                BTN_2_3 => self.q_events.enqueue(Event::KeyItem(KeyEvent::Btn23)),
                BTN_3_ESC => self.q_events.enqueue(Event::KeyItem(KeyEvent::Btn3Esc)),
                BTN_1_ESC => self.q_events.enqueue(Event::KeyItem(KeyEvent::Btn1Esc)),
                BTN_1_ESC_ENC => self.q_events.enqueue(Event::KeyItem(KeyEvent::Btn1EscEnc)),
                _ => Ok(()),
            };
            self.first_go_to_0 = true;
            self.tick_cnt = 0;
        } else if btn_state > 0 {
            self.tick_cnt += 1;
            // Triggers when keys are pressed for more then 3 seconds
            if self.tick_cnt > 60 {
                let _ = match btn_state {
                    BTN_1 => self.q_events.enqueue(Event::KeyItem(KeyEvent::Btn1S3)),
                    BTN_2 => self.q_events.enqueue(Event::KeyItem(KeyEvent::Btn2S3)),
                    BTN_3 => self.q_events.enqueue(Event::KeyItem(KeyEvent::Btn3S3)),
                    BTN_ESC => self.q_events.enqueue(Event::KeyItem(KeyEvent::BtnEscS3)),
                    BTN_ENC => self.q_events.enqueue(Event::KeyItem(KeyEvent::BtnEncS3)),
                    BTN_1_2 => self.q_events.enqueue(Event::KeyItem(KeyEvent::Btn12S3)),
                    BTN_2_3 => self.q_events.enqueue(Event::KeyItem(KeyEvent::Btn23S3)),
                    BTN_3_ESC => self.q_events.enqueue(Event::KeyItem(KeyEvent::Btn3EscS3)),
                    BTN_1_ESC => self.q_events.enqueue(Event::KeyItem(KeyEvent::Btn1EscS3)),
                    BTN_1_ESC_ENC => self
                        .q_events
                        .enqueue(Event::KeyItem(KeyEvent::Btn1EscEncS3)),
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
                    .enqueue(Event::KeyItem(KeyEvent::Rotary1Right));
                delta -= 1;
            }
            while delta < 0 {
                let _ = self.q_events.enqueue(Event::KeyItem(KeyEvent::Rotary1Left));
                delta += 1;
            }
            self.enc_1_cnt = count;
        }

        let count = self.tim_enc_2.cnt.read().cnt().bits();

        if count != self.enc_2_cnt {
            let mut delta = count.wrapping_sub(self.enc_2_cnt) as i32;
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
            self.enc_2_cnt = count;
        }
    }
}
