#![allow(unused)]
use embassy_sync::{blocking_mutex::raw::{CriticalSectionRawMutex, ThreadModeRawMutex}, mutex::Mutex, pubsub::PubSubChannel};

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
    pub deadband_in_min: i16, // mv no throttle
    pub deadband_in_max: i16, // mv full throttle
    pub deadband_out_min: i16, // mv just before motor active
    pub deadband_out_max: i16,
    pub speed_limit: i16, // mv just after max speed, or supply voltage
    pub limit_min: i16, // mv min (cruise control?) // TODO: remove?
    pub limit_max: i16, // mv max (speed limit) // TODO: remove?
}

/*
Controller supply voltage - 4.36v = 4360mv
*/
pub static THROTTLE_SETTINGS: Mutex<SettingsMutex, ThrottleSettings> = Mutex::new(ThrottleSettings {
    passthrough: false, // disable smoothing and limiting
    increase_smooth_factor: 4000, // rate of smoothing to acceleration
    decrease_smooth_factor: 100, // rate of smoothing to deceleration
    deadband_in_min: 847, // mv no throttle
    deadband_in_max: 3580, // mv full throttle
    deadband_out_min: 1230, // mv just before motor active
    deadband_out_max: 2600, // mv just after max speed, or supply voltage
    speed_limit: 1023, // as mv
    limit_min: 100, // mv min (cruise control?)
    limit_max: 1023, // mv max (speed limit)
});