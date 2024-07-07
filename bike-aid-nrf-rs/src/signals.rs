#![allow(unused)]

use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, pubsub::PubSubChannel};

type ChannelMutex = CriticalSectionRawMutex;

// External / reporting to user

pub static SYSTEM_POWER: PubSubChannel<ChannelMutex, bool, 2, 2, 2> = PubSubChannel::new();

pub static ALARM: PubSubChannel<ChannelMutex, bool, 2, 2, 2> = PubSubChannel::new();

pub static CLOCK_HOURS: PubSubChannel<ChannelMutex, u8, 2, 2, 2> = PubSubChannel::new();
pub static CLOCK_MINUTES: PubSubChannel<ChannelMutex, u8, 2, 2, 2> = PubSubChannel::new();

pub static INSTANT_SPEED: PubSubChannel<ChannelMutex, u32, 2, 2, 2> = PubSubChannel::new();
pub static SMOOTH_SPEED: PubSubChannel<ChannelMutex, u32, 2, 2, 2> = PubSubChannel::new();
pub static WHEEL_ROTATIONS: PubSubChannel<ChannelMutex, u8, 2, 2, 2> = PubSubChannel::new();
pub static ODOMETER: PubSubChannel<ChannelMutex, u8, 2, 2, 2> = PubSubChannel::new();

pub static TEMPERATURE: PubSubChannel<ChannelMutex, u16, 2, 2, 2> = PubSubChannel::new();

pub static BATTERY_CURRENT: PubSubChannel<ChannelMutex, u8, 2, 2, 2> = PubSubChannel::new();
pub static BATTERY_VOLTAGE: PubSubChannel<ChannelMutex, u8, 2, 2, 2> = PubSubChannel::new();
pub static BATTERY_POWER: PubSubChannel<ChannelMutex, u8, 2, 2, 2> = PubSubChannel::new();

// Internal

pub type LedModeType = crate::task_led::LedMode;
pub static LED_MODE: PubSubChannel<ChannelMutex, LedModeType, 2, 2, 2> = PubSubChannel::new();

pub static BUTTON_ON: PubSubChannel<ChannelMutex, bool, 2, 2, 2> = PubSubChannel::new();

pub static THROTTLE_IN: PubSubChannel<ChannelMutex, i16, 2, 2, 2> = PubSubChannel::new();
pub static THROTTLE_OUT: PubSubChannel<ChannelMutex, i16, 2, 2, 2> = PubSubChannel::new();
