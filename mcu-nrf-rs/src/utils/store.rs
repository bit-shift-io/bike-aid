#![allow(unused)]
use embassy_sync::{blocking_mutex::raw::{CriticalSectionRawMutex, ThreadModeRawMutex}, mutex::Mutex, pubsub::PubSubChannel};

// TODO: investigate supposed to use async mutex for async functions
// https://github.com/embassy-rs/embassy/blob/main/examples/rp/src/bin/sharing.rs
// https://docs.rs/scoped-mutex/latest/scoped_mutex/struct.BlockingMutex.html <-- comming soon no refcell required!

type SettingsMutex = ThreadModeRawMutex;


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
    pub increase_smooth_factor: i16, // rate of smoothing to acceleration
    pub decrease_smooth_factor: i16, // rate of smoothing to deceleration
    pub no_throttle: i16, // mv no throttle
    pub full_throttle: i16, // mv full throttle
    pub deadband_min: i16, // mv just before motor active
    pub deadband_max: i16,
    pub speed_limit: i16, // mv just after max speed, or supply voltage
}

/*
Controller supply voltage - 4.36v = 4360mv
*/
pub static THROTTLE_SETTINGS: Mutex<SettingsMutex, ThrottleSettings> = Mutex::new(ThrottleSettings {
    passthrough: false, // disable smoothing and limiting
    increase_smooth_factor: 4000, // rate of smoothing to acceleration
    decrease_smooth_factor: 100, // rate of smoothing to deceleration
    no_throttle: 847, // mv no throttle
    full_throttle: 3580, // mv full throttle
    deadband_min: 1230, // mv just before motor active
    deadband_max: 2600, // mv just after max speed, or supply voltage
    speed_limit: 1023, // as mv
});