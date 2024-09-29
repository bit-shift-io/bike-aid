#![allow(unused)]

use embassy_sync::{blocking_mutex::raw::{CriticalSectionRawMutex, ThreadModeRawMutex}, mutex::Mutex, pubsub::PubSubChannel};
use heapless::{pool::boxed::Box, String};
use nrf_softdevice::ble::Connection;

// configure types
type SettingsMutex = ThreadModeRawMutex;
type ChannelMutex = CriticalSectionRawMutex;

// == CHANNELS ==
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
pub static ODOMETER: PubSubChannel<ChannelMutex, u16, 1, 2, 2> = PubSubChannel::new();

pub static TEMPERATURE: PubSubChannel<ChannelMutex, u8, 1, 2, 2> = PubSubChannel::new();

pub static BATTERY_CURRENT: PubSubChannel<ChannelMutex, u16, 1, 2, 2> = PubSubChannel::new();
pub static BATTERY_VOLTAGE: PubSubChannel<ChannelMutex, u16, 1, 2, 2> = PubSubChannel::new();
pub static BATTERY_POWER: PubSubChannel<ChannelMutex, u16, 1, 2, 2> = PubSubChannel::new();
pub static BATTERY_LEVEL: PubSubChannel<ChannelMutex, u8, 1, 1, 1> = PubSubChannel::new();

// Internal
pub static BATTERY_IN: PubSubChannel<ChannelMutex, [u16;2], 1, 2, 2> = PubSubChannel::new();

pub type LedModeType = crate::tasks::led::LedMode;
pub static LED_MODE: PubSubChannel<ChannelMutex, LedModeType, 1, 2, 2> = PubSubChannel::new();

pub type PiezoModeType = crate::tasks::piezo::PiezoMode;
pub static PIEZO_MODE: PubSubChannel<ChannelMutex, PiezoModeType, 1, 1, 2> = PubSubChannel::new();

pub static BRAKE_ON: PubSubChannel<ChannelMutex, bool, 1, 2, 2> = PubSubChannel::new();

// alarm
pub static ALARM_ENABLED: PubSubChannel<ChannelMutex, bool, 1, 2, 2> = PubSubChannel::new();
pub static ALARM_ALERT_ACTIVE: PubSubChannel<ChannelMutex, bool, 1, 2, 2> = PubSubChannel::new();
pub static ALARM_MOTION_DETECTED: PubSubChannel<ChannelMutex, bool, 1, 2, 2> = PubSubChannel::new();

// throttle
pub static THROTTLE_SETTINGS_CHANGE: PubSubChannel<ChannelMutex, u16, 1, 2, 2> = PubSubChannel::new();
pub static THROTTLE_IN: PubSubChannel<ChannelMutex, u16, 1, 2, 2> = PubSubChannel::new();
pub static THROTTLE_OUT: PubSubChannel<ChannelMutex, u16, 1, 2, 2> = PubSubChannel::new();

// ble uart
const MAX_LENGTH: usize = 32;
//pub static TEST: PubSubChannel<ChannelMutex, &[u8], 1, 2, 2> = PubSubChannel::new();
pub static UART_WRITE: PubSubChannel<ChannelMutex, String<MAX_LENGTH>, 1, 2, 2> = PubSubChannel::new();
pub static UART_READ: PubSubChannel<ChannelMutex, String<MAX_LENGTH>, 1, 2, 2> = PubSubChannel::new();

// settings changed, write to flash
pub static STORE_WRITE: PubSubChannel<ChannelMutex, bool, 1, 2, 2> = PubSubChannel::new();
pub static STORE_UPDATED: PubSubChannel<ChannelMutex, bool, 1, 2, 2> = PubSubChannel::new();




// == MUTEX'S ==


// TODO: investigate supposed to use async mutex for async functions
// https://github.com/embassy-rs/embassy/blob/main/examples/rp/src/bin/sharing.rs
// https://docs.rs/scoped-mutex/latest/scoped_mutex/struct.BlockingMutex.html <-- comming soon no refcell required!




pub static SYSTEM_POWER_ACTIVE: Mutex<SettingsMutex, bool> = Mutex::new(false);

pub struct AlarmSettings {
    pub active: bool,
    pub warnings: u8,
    pub warning_interval: u8,
    pub sensitivity: u8,
}
pub static ALARM_ACTIVE: Mutex<SettingsMutex, bool> = Mutex::new(false);


/* 
Deadband / Deadzone
===========================
Adjust throttle range to eliminate deadband/deadzones. 
All the ranges below can be determined by watching the serial console and twisting the throttle.
Or use a multimeter to measure voltage output from the throttle on your ebike.

IN_MIN - Voltage when the throttle is unpressed
IN_MAX - Voltage when the throttle is fully pressed
OUT_MIN - Voltage just before the motor starts to activate the wheels
OUT_MAX - Voltage just after max speed (or use supply voltage otherwise)

Smoothing - Jerkiness Mitigation
===========================
How quickly to adjust output over time.
Larger values are slower and smoother, smaller are more responsive.

Speed Limit
===========================
Adjusts throttle output speed limit.
*/

pub struct ThrottleSettings {
    pub passthrough: bool, // disable smoothing and limiting
    pub increase_smooth_factor: u16, // rate of smoothing to acceleration
    pub decrease_smooth_factor: u16, // rate of smoothing to deceleration
    pub throttle_min: u16, // mv no throttle
    pub throttle_max: u16, // mv full throttle
    pub deadband_min: u16, // mv just before motor active
    pub deadband_max: u16,
    pub speed_limit: u16, // mv just after max speed, or supply voltage
}


/*
Controller supply voltage - 4.36v = 4360mv
*/
pub static THROTTLE_SETTINGS: Mutex<SettingsMutex, ThrottleSettings> = Mutex::new(ThrottleSettings {
    passthrough: false, // disable smoothing and limiting
    increase_smooth_factor: 75, // rate of smoothing to acceleration
    decrease_smooth_factor: 120, // rate of smoothing to deceleration
    throttle_min: 910, // mv no throttle
    throttle_max: 3400, // mv full throttle
    deadband_min: 1200, // mv just before motor active
    deadband_max: 2000, // mv just after max speed, or supply voltage
    speed_limit: 4000, // as mv
});