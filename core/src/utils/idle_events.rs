use crate::{DateTime, PersistenceId, PersistenceItem};
use heapless::spsc::{Consumer, Producer, Queue};

#[derive(Debug, Copy, Clone)]
pub enum IdleEvent {
    SetEepromItem(PersistenceItem),
    ClearEepromItems(&'static[PersistenceId]),
    SdCardItem(SdCardCmd),
    FeedTheDog,
    SetGain(u8),
    DateTime(DateTime),
    ResetDevice,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SdCardCmd {
    SwUpdateAccepted,
    SwUpdateCanceled,
}

// This queue transports the configuration PersItems from controller to the idle loop.
const MAX_IDLE_EVENTS: usize = 20;
pub type QIdleEvents = Queue<IdleEvent, MAX_IDLE_EVENTS>;
pub type PIdleEvents = Producer<'static, IdleEvent, MAX_IDLE_EVENTS>;
pub type CIdleEvents = Consumer<'static, IdleEvent, MAX_IDLE_EVENTS>;
