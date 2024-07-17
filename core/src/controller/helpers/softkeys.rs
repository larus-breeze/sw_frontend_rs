use crate::{utils::KeyEvent, Editable};

#[derive(Debug, Clone, Copy)]
pub struct Softkeys {
    primary: [Editable; 4],
    key1_3s: Editable,
    key2_3s: Editable,
    fallback: Editable,
    fallback_activated: bool,
    current: Editable,
    curr_idx: u8,
}

impl Softkeys {
    pub fn new() -> Self {
        Softkeys {
            primary: [Editable::None; 4],
            key1_3s: Editable::None,
            key2_3s: Editable::None,
            fallback: Editable::Volume,
            fallback_activated: true,
            current: Editable::Volume,
            curr_idx: 0,
        }
    }

    pub fn current(&self) -> Editable {
        self.current
    }

    pub fn set_editables(
        &mut self,
        key1: Editable,
        key2: Editable,
        key3: Editable,
        key4: Editable,
    ) {
        self.primary = [key1, key2, key3, key4];
    }

    pub fn set_fallback(&mut self, fallback: Editable) {
        self.fallback = fallback
    }

    pub fn set_3s_keys(&mut self, key1_3s: Editable, key2_3s: Editable) {
        self.key1_3s = key1_3s;
        self.key2_3s = key2_3s;
    }

    pub fn to_fallback(&mut self) {
        self.current = self.fallback;
        self.fallback_activated = true;
    }

    /// Interpret keyboard event
    ///
    /// The result is true, if the editor has to show the edit window or refresh the timer for it
    pub fn key_action(&mut self, event: &KeyEvent) -> bool {
        let editable = match event {
            KeyEvent::Btn1 => {
                if self.current == Editable::Volume {
                    self.curr_idx = 0;
                } else {
                    self.curr_idx = (self.curr_idx.wrapping_add(3)) % 4;
                }
                self.primary[self.curr_idx as usize]
            }
            KeyEvent::Btn2 => {
                if self.current == Editable::Volume {
                    self.curr_idx = 1;
                } else {
                    self.curr_idx = (self.curr_idx.wrapping_add(1)) % 4;
                }
                self.primary[self.curr_idx as usize]
            }
            KeyEvent::Btn3 => {
                self.curr_idx = 2;
                self.primary[self.curr_idx as usize]
            }
            KeyEvent::BtnEsc => {
                self.curr_idx = 3;
                self.primary[self.curr_idx as usize]
            }
            KeyEvent::Btn1S3 => self.key1_3s,
            KeyEvent::Btn2S3 => self.key2_3s,
            KeyEvent::Rotary1Left | KeyEvent::Rotary1Right => {
                if self.current == Editable::Volume {
                    Editable::Display
                } else {
                    self.current
                }
            }
            KeyEvent::Rotary2Left | KeyEvent::Rotary2Right => {
                if self.current == Editable::Display {
                    Editable::Volume
                } else {
                    self.current
                }
            }
            _ => self.current,
        };

        let r = (editable != self.current) || self.fallback_activated;
        self.current = editable;
        self.fallback_activated = false;
        r
    }
}
