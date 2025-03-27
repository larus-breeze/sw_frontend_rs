use crate::{DateTime, PersistenceId, PersistenceItem};
use heapless::spsc::{Consumer, Producer, Queue};

#[derive(Debug, Copy, Clone)]
pub enum IdleEvent {
    SetEepromItem(PersistenceItem),
    ClearEepromItems(&'static [PersistenceId]),
    SdCardItem(SdCardCmd),
    FeedTheDog,
    SetGain(u8),
    DateTime(DateTime),
    ResetDevice(ResetReason),
    Output1(PinState),
    Output2(PinState),
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PinState {
    High,
    Low,
}

impl core::ops::Not for PinState {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            PinState::High => PinState::Low,
            PinState::Low => PinState::High
        }
    }
}

impl core::convert::From<bool> for PinState {
    fn from(is_high: bool) -> Self {
        if is_high {
            PinState::High
        } else {
            PinState::Low
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ResetReason {
    NoReason,
    ConfigChanged,
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
