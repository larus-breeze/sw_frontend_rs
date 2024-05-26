use crate::{
    Editable,
    utils::KeyEvent,
};

#[derive(Debug, Clone, Copy)]
pub struct Softkeys {
    primary: [Editable; 4],
    key1_3s: Editable,
    key2_3s: Editable,
    fallback: Editable,
    current: Editable,
    curr_idx: u8,
}

impl Softkeys {
    pub fn new() -> Self {
      Softkeys {
        primary: [Editable::None; 4],
        key1_3s: Editable::Glider,
        key2_3s: Editable::Theme,
        fallback: Editable::Volume,
        current: Editable::Volume,
        curr_idx: 0,
      }
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
    }

    pub fn key_action(&mut self, event: KeyEvent) -> Editable {
        let mut set_new = true;
        let editable = match event {
            KeyEvent::Btn1 => {
                if self.current == Editable::Volume {
                    self.curr_idx = 0;                    
                } else {
                    self.curr_idx = (self.curr_idx.wrapping_add(3)) % 4; 
                }
                None
            }
            KeyEvent::Btn2 => {
                if self.current == Editable::Volume {
                    self.curr_idx = 1;                    
                } else {
                    self.curr_idx = (self.curr_idx.wrapping_add(1)) % 4; 
                }
                None
            }
            KeyEvent::Btn3 => {
                self.curr_idx = 2; 
                None
            }
            KeyEvent::BtnEsc => {
                self.curr_idx = 3; 
                None
            }
            KeyEvent::Btn1S3 => {
                Some(self.key1_3s)
            }
            KeyEvent::Btn2S3 => {
                Some(self.key2_3s)
            }
            _ => {
                set_new = false;
                Some(self.current)
            }
        };
        self.current = if set_new {
            if editable == None {
                self.primary[self.curr_idx as usize]
            } else {
                editable.unwrap()
            }
        } else {
            self.current
        };
        self.current
    }



}