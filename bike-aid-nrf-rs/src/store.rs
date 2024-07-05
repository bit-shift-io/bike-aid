#![allow(unused)]

use embassy_sync::{blocking_mutex::raw::{CriticalSectionRawMutex, ThreadModeRawMutex}, mutex::Mutex, pubsub::PubSubChannel};

// https://docs.embassy.dev/embassy-sync/git/default/blocking_mutex/struct.Mutex.html
type SettingsMutex = ThreadModeRawMutex;

pub static SYSTEM_POWER_ACTIVE: Mutex<SettingsMutex, bool> = Mutex::new(true);

pub static ALARM_ACTIVE: Mutex<SettingsMutex, bool> = Mutex::new(false);


// THROTTLE
pub static THROTTLE_INCREASE_SMOOTH_FACTOR: Mutex<SettingsMutex, i16> = Mutex::new(4000);
pub static THROTTLE_DECREASE_SMOOTH_FACTOR: Mutex<SettingsMutex, i16> = Mutex::new(100);

pub static THROTTLE_LIMIT_MIN: Mutex<SettingsMutex, i16> = Mutex::new(100); // mv min (cruise control?)
pub static THROTTLE_LIMIT_MAX: Mutex<SettingsMutex, i16> = Mutex::new(1023); // mv max (speed limit) // TODO: change to use speed eg 20khr

// controller supply voltage - 4.36v = 4360mv
pub static THROTTLE_DEADBAND_IN_MIN: Mutex<SettingsMutex, i16> = Mutex::new(847); // mv no throttle
pub static THROTTLE_DEADBAND_IN_MAX: Mutex<SettingsMutex, i16> = Mutex::new(3580); // mv full throttle
pub static THROTTLE_DEADBAND_OUT_MIN: Mutex<SettingsMutex, i16> = Mutex::new(1230); // mv just before motor active
pub static THROTTLE_DEADBAND_OUT_MAX: Mutex<SettingsMutex, i16> = Mutex::new(2600); // mv just after max speed