#![allow(unused)]

use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, pubsub::PubSubChannel};
use heapless::{pool::boxed::Box, String};
use nrf_softdevice::ble::Connection;

type ChannelMutex = CriticalSectionRawMutex;
// <Mutex Type, Data Type, Max Channels(History), Max Subscribers, Max Publishers>

// External / reporting to user

pub static SWITCH_POWER: PubSubChannel<ChannelMutex, bool, 1, 9, 2> = PubSubChannel::new();
pub static SWITCH_HORN: PubSubChannel<ChannelMutex, bool, 1, 2, 2> = PubSubChannel::new();
pub static SWITCH_LIGHT: PubSubChannel<ChannelMutex, bool, 1, 2, 2> = PubSubChannel::new();

pub static CLOCK_HOURS: PubSubChannel<ChannelMutex, u8, 1, 2, 2> = PubSubChannel::new();
pub static CLOCK_MINUTES: PubSubChannel<ChannelMutex, u8, 1, 2, 2> = PubSubChannel::new();

pub static INSTANT_SPEED: PubSubChannel<ChannelMutex, u32, 1, 2, 2> = PubSubChannel::new();
pub static SMOOTH_SPEED: PubSubChannel<ChannelMutex, u8, 1, 2, 2> = PubSubChannel::new();
pub static WHEEL_ROTATIONS: PubSubChannel<ChannelMutex, u8, 1, 2, 2> = PubSubChannel::new();
pub static ODOMETER: PubSubChannel<ChannelMutex, u8, 1, 2, 2> = PubSubChannel::new();

pub static TEMPERATURE: PubSubChannel<ChannelMutex, u8, 1, 2, 2> = PubSubChannel::new();

pub static BATTERY_CURRENT: PubSubChannel<ChannelMutex, i16, 1, 2, 2> = PubSubChannel::new();
pub static BATTERY_VOLTAGE: PubSubChannel<ChannelMutex, i16, 1, 2, 2> = PubSubChannel::new();
pub static BATTERY_POWER: PubSubChannel<ChannelMutex, i16, 1, 2, 2> = PubSubChannel::new();

// Internal
pub static BATTERY_IN: PubSubChannel<ChannelMutex, [i16;2], 1, 2, 2> = PubSubChannel::new();

pub type LedModeType = crate::tasks::led::LedMode;
pub static LED_MODE: PubSubChannel<ChannelMutex, LedModeType, 1, 2, 2> = PubSubChannel::new();

pub static BRAKE_ON: PubSubChannel<ChannelMutex, bool, 1, 2, 2> = PubSubChannel::new();

// alarm
pub static ALARM_ENABLED: PubSubChannel<ChannelMutex, bool, 1, 2, 2> = PubSubChannel::new();
pub static ALARM_ALERT_ACTIVE: PubSubChannel<ChannelMutex, bool, 1, 2, 2> = PubSubChannel::new();
pub static ALARM_MOTION_DETECTED: PubSubChannel<ChannelMutex, bool, 1, 2, 2> = PubSubChannel::new();

// throttle
pub static THROTTLE_SETTINGS_CHANGE: PubSubChannel<ChannelMutex, i16, 1, 2, 2> = PubSubChannel::new();
pub static THROTTLE_IN: PubSubChannel<ChannelMutex, i16, 1, 2, 2> = PubSubChannel::new();
pub static THROTTLE_OUT: PubSubChannel<ChannelMutex, i16, 1, 2, 2> = PubSubChannel::new();

// ble uart
const MAX_LENGTH: usize = 32;
pub static UART_WRITE: PubSubChannel<ChannelMutex, String<MAX_LENGTH>, 1, 2, 2> = PubSubChannel::new();
pub static UART_READ: PubSubChannel<ChannelMutex, String<MAX_LENGTH>, 1, 2, 2> = PubSubChannel::new();

// settings changed, write to flash
pub static STORE_WRITE: PubSubChannel<ChannelMutex, bool, 1, 2, 2> = PubSubChannel::new();
pub static STORE_UPDATED: PubSubChannel<ChannelMutex, bool, 1, 2, 2> = PubSubChannel::new();