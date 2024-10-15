#![allow(unused)]
use embassy_sync::{blocking_mutex::raw::{CriticalSectionRawMutex, ThreadModeRawMutex}, mutex::Mutex, pubsub::PubSubChannel, watch::Watch};
use heapless::{pool::boxed::Box, String};
use nrf_softdevice::ble::Connection;

// configure types
type SignalMutex = ThreadModeRawMutex;
type ChannelMutex = CriticalSectionRawMutex;
type WatchMutex = CriticalSectionRawMutex;

// default values
pub fn init() {
    BRAKE_ON_WATCH.dyn_sender().send_if_modified(|value| { *value = Some(false); false });
    PARK_BRAKE_ON_WATCH.dyn_sender().send_if_modified(|value| { *value = Some(true); false });
    CRUISE_LEVEL_WATCH.dyn_sender().send_if_modified(|value| { *value = Some(0u8); false });
    CLOCK_HOURS_WATCH.dyn_sender().send_if_modified(|value| { *value = Some(0u8); false });
    CLOCK_MINUTES_WATCH.dyn_sender().send_if_modified(|value| { *value = Some(0u8); false });
    THROTTLE_IN_WATCH.dyn_sender().send_if_modified(|value| { *value = Some(0u16); false });
    THROTTLE_OUT_WATCH.dyn_sender().send_if_modified(|value| { *value = Some(0u16); false });
    TEMPERATURE_WATCH.dyn_sender().send_if_modified(|value| { *value = Some(0u8); false });
}



// == WATCHES ===
// Watches can not have history
// Return None if max receivers is reached
pub static BRAKE_ON_WATCH: Watch<WatchMutex, bool, 3> = Watch::new();
pub static PARK_BRAKE_ON_WATCH: Watch<WatchMutex, bool, 4> = Watch::new();
pub static CRUISE_LEVEL_WATCH: Watch<WatchMutex, u8, 2> = Watch::new();
pub static CLOCK_HOURS_WATCH: Watch<WatchMutex, u8, 1> = Watch::new();
pub static CLOCK_MINUTES_WATCH: Watch<WatchMutex, u8, 1> = Watch::new();
pub static THROTTLE_IN_WATCH: Watch<WatchMutex, u16, 3> = Watch::new();
pub static THROTTLE_OUT_WATCH: Watch<WatchMutex, u16, 1> = Watch::new();
pub static TEMPERATURE_WATCH: Watch<WatchMutex, u8, 1> = Watch::new();
pub static POWER_ON_WATCH: Watch<WatchMutex, bool, 4> = Watch::new();



// == CHANNELS ==
// Channels can have history
// <Mutex Type, Data Type, Max Channels(History), Max Subscribers, Max Publishers>

// External / reporting to user

pub static SWITCH_HORN: PubSubChannel<ChannelMutex, bool, 1, 2, 2> = PubSubChannel::new();
pub static SWITCH_LIGHT: PubSubChannel<ChannelMutex, bool, 1, 2, 2> = PubSubChannel::new();

pub static INSTANT_SPEED: PubSubChannel<ChannelMutex, u32, 1, 2, 2> = PubSubChannel::new();
pub static SMOOTH_SPEED: PubSubChannel<ChannelMutex, u8, 1, 2, 2> = PubSubChannel::new();
pub static WHEEL_ROTATIONS: PubSubChannel<ChannelMutex, u8, 1, 2, 2> = PubSubChannel::new();
pub static ODOMETER: PubSubChannel<ChannelMutex, u16, 1, 2, 2> = PubSubChannel::new();

pub static BATTERY_CURRENT: PubSubChannel<ChannelMutex, u16, 1, 2, 2> = PubSubChannel::new();
pub static BATTERY_VOLTAGE: PubSubChannel<ChannelMutex, u16, 1, 2, 2> = PubSubChannel::new();
pub static BATTERY_POWER: PubSubChannel<ChannelMutex, u16, 1, 2, 2> = PubSubChannel::new();
pub static BATTERY_LEVEL: PubSubChannel<ChannelMutex, u8, 1, 1, 1> = PubSubChannel::new();

pub static BATTERY_IN: PubSubChannel<ChannelMutex, [u16;2], 1, 2, 2> = PubSubChannel::new();

pub type LedModeType = crate::tasks::led::LedMode;
pub static LED_MODE: PubSubChannel<ChannelMutex, LedModeType, 1, 2, 2> = PubSubChannel::new();

pub type PiezoModeType = crate::tasks::piezo::PiezoMode;
pub static PIEZO_MODE: PubSubChannel<ChannelMutex, PiezoModeType, 1, 1, 6> = PubSubChannel::new();


// alarm
pub static ALARM_ENABLED: PubSubChannel<ChannelMutex, bool, 1, 3, 2> = PubSubChannel::new();
pub static ALARM_ALERT_ACTIVE: PubSubChannel<ChannelMutex, bool, 1, 2, 2> = PubSubChannel::new();
pub static ALARM_MOTION_DETECTED: PubSubChannel<ChannelMutex, bool, 1, 2, 2> = PubSubChannel::new();

// ble uart
const MAX_LENGTH: usize = 32;
//pub static TEST: PubSubChannel<ChannelMutex, &[u8], 1, 2, 2> = PubSubChannel::new();
pub static UART_WRITE: PubSubChannel<ChannelMutex, String<MAX_LENGTH>, 1, 2, 2> = PubSubChannel::new();
pub static UART_READ: PubSubChannel<ChannelMutex, String<MAX_LENGTH>, 1, 2, 2> = PubSubChannel::new();

// settings changed, write to flash
pub static STORE_WRITE: PubSubChannel<ChannelMutex, bool, 1, 2, 2> = PubSubChannel::new();
pub static STORE_UPDATED: PubSubChannel<ChannelMutex, bool, 1, 2, 2> = PubSubChannel::new();




// == MUTEX'S ==
// Mutex dont notify, only for use in loops

pub static CRUISE_VOLTAGE: Mutex<SignalMutex, u16> = Mutex::new(0u16);
pub static CRUISE_VOLTAGES: Mutex<SignalMutex, [u16;5]> = Mutex::new([
        1600u16, // 1408 a little too slow, 1500 a little slow
        2000u16, // 1906 a little to slow
        2300u16, // 2400 a little fast?
        2800u16, // 2900 ?
        3400u16, // 3400 max
    ]);

pub struct AlarmSettings {
    pub active: bool,
    pub warnings: u8,
    pub warning_interval: u8,
    pub sensitivity: u8,
}
pub static ALARM_ACTIVE: Mutex<SignalMutex, bool> = Mutex::new(false);


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

#[derive(Clone, Copy)]
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
pub static THROTTLE_SETTINGS: Mutex<SignalMutex, ThrottleSettings> = Mutex::new(ThrottleSettings {
    passthrough: false, // disable smoothing and limiting
    increase_smooth_factor: 75, // rate of smoothing to acceleration
    decrease_smooth_factor: 150, // rate of smoothing to deceleration
    throttle_min: 900, // mv no throttle
    throttle_max: 3600, // mv full throttle
    deadband_min: 1200, // mv just before motor active
    deadband_max: 2000, // mv just after max speed, or supply voltage
    speed_limit: 4000, // as mv
});