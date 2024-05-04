use crate::{eeprom, DateTime, PersistenceItem};
use heapless::spsc::{Consumer, Producer, Queue};

#[repr(u16)]
#[derive(Debug, Copy, Clone)]
pub enum IdleEvent {
    EepromItem(PersistenceItem),
    SdCardItem(SdCardCmd),
    FeedTheDog,
    SetGain(u8),
    DateTime(DateTime),
}

#[repr(u16)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SdCardCmd {
    SwUpdateAccepted,
    SwUpdateCanceled,
}

#[repr(u16)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum PersistenceId {
    DoNotStore = 65535,
    Volume = 0,
    McCready = 1,
    WaterBallast = 2,
    PilotWeight = 3,
    Glider = 4,
    VarioModeControl = 5,
    DisplayMode = 6, // Dark = 0, Bright = 1
    Qnh = 7,
    Bugs = 8,
    LastItem,
}

// This queue transports the configuration PersItems from controller to the idle loop.
const MAX_IDLE_EVENTS: usize = 20;
pub type QIdleEvents = Queue<IdleEvent, MAX_IDLE_EVENTS>;
pub type PIdleEvents = Producer<'static, IdleEvent, MAX_IDLE_EVENTS>;
pub type CIdleEvents = Consumer<'static, IdleEvent, MAX_IDLE_EVENTS>;

impl From<u16> for PersistenceId {
    fn from(src: u16) -> Self {
        if src < eeprom::MAX_ITEM_COUNT as u16 && src < PersistenceId::LastItem as u16 {
            // Safety: Only valid or possible values are transmuted
            unsafe { core::mem::transmute::<u16, PersistenceId>(src) }
        } else {
            panic!()
        }
    }
}
