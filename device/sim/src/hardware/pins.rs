use corelib::*;

pub struct OutPins {
    out1: PinState,
    out2: PinState,
}

impl OutPins {
    pub fn new() -> Self {
        OutPins { out1: PinState::High, out2: PinState::High }
    }

    pub fn set_state(&mut self, event: IdleEvent) -> Option<String> {
        let (pin_name, state) = match event {
            IdleEvent::Output1(state) => {
                self.out1 = state;
                ("Flash Control", (!self.out1).as_str()) // hw inverts polarity
            },
            IdleEvent::Output2(state) => {
                self.out2 = state;
                ("?", (!self.out2).as_str())
            },
            _ => return None
        };
        Some(format!("{}: {}", pin_name, state))
    }
}


pub struct InPins {
    in1: PinState,
    in2: PinState,
    in3: PinState,
    in4: PinState,
}

impl InPins {
    pub fn new() -> Self {
        InPins { 
            in1: PinState::Low, 
            in2: PinState::Low, 
            in3: PinState::Low, 
            in4: PinState::Low 
        }
    }

    pub fn toggle(&mut self, pin_name: &str) {
        match pin_name {
            "2" => self.in2 = !self.in2,
            "3" => self.in3 = !self.in3,
            "4" => self.in4 = !self.in4,
            _ => self.in1 = !self.in1,
        }
    }

    pub fn button_text(&mut self, pin_name: &str) -> String {
        let name = match pin_name {
            "2" => "Sft",
            "3" => "Gear",
            "4" => "Break",
            _ => "Water",
        };
        let state = match pin_name {
            "2" => self.in2.as_str(),
            "3" => self.in3.as_str(),
            "4" => self.in4.as_str(),
            _ => self.in1.as_str(),
        };
        format!("{}: {}", name, state)
    }

    pub fn event(&mut self, pin_name: &str) -> Event {
        match pin_name {
            "2" => Event::InputItem(InputPinState::Io2(self.in2)),
            "3" => Event::InputItem(InputPinState::Io3(self.in3)),
            "4" => Event::InputItem(InputPinState::Io4(self.in4)),
            _ => Event::InputItem(InputPinState::Io1(self.in1)),
        }
    }
}
