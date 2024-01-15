use crate::{eeprom, PersistenceItem};
use heapless::spsc::{Consumer, Producer, Queue};

#[repr(u16)]
#[derive(Debug, Copy, Clone)]
pub enum IdleEvent {
    EepromItem(PersistenceItem),
    SdCardItem(SdCardCmd),
    FeedTheDog,
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
}

// This queue transports the configuration PersItems from controller to the idle loop.
const MAX_IDLE_EVENTS: usize = 20;
pub type QIdleEvents = Queue<IdleEvent, MAX_IDLE_EVENTS>;
pub type PIdleEvents = Producer<'static, IdleEvent, MAX_IDLE_EVENTS>;
pub type CIdleEvents = Consumer<'static, IdleEvent, MAX_IDLE_EVENTS>;

impl From<u16> for PersistenceId {
    fn from(src: u16) -> Self {
        if src < eeprom::MAX_ITEM_COUNT as u16 {
            // Safety: Only valid or possible values are transmuted
            unsafe { core::mem::transmute::<u16, PersistenceId>(src) }
        } else {
            panic!()
        }
    }
}
