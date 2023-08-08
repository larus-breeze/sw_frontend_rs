
#[repr(u16)]
pub enum ConfigId {
    Magic = 0,
    DisplayActive = 1,
    Volume = 2,
    McCready = 3,
    EmptyWeight = 4,
    PilotWeight = 5,
    WaterBallast = 6,
    Bugs = MAX,
}

const MAX: u16 = 7; 

impl From<u16> for ConfigId {
    fn from(id: u16) -> Self {
        if id <= MAX {
            // We checked the range, so transmute is ok
            unsafe{core::mem::transmute::<u16, ConfiId>(id)}
        } else {
            core::panic!()
        }
    }
}