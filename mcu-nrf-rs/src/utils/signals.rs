#![allow(unused)]
use embassy_sync::{blocking_mutex::raw::{CriticalSectionRawMutex, ThreadModeRawMutex}, mutex::Mutex, pubsub::PubSubChannel, watch::Watch};
use heapless::{pool::boxed::Box, String};
use nrf_softdevice::ble::Connection;
use embassy_sync::priority_channel::{PriorityChannel, Max};

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
    POWER_ON_WATCH.dyn_sender().send_if_modified(|value| { *value = Some(false); false });
    SWITCH_HORN_WATCH.dyn_sender().send_if_modified(|value| { *value = Some(false); false });
    SWITCH_LIGHT_WATCH.dyn_sender().send_if_modified(|value| { *value = Some(false); false });
    INSTANT_SPEED_WATCH.dyn_sender().send_if_modified(|value| { *value = Some(0u32); false });
    SMOOTH_SPEED_WATCH.dyn_sender().send_if_modified(|value| { *value = Some(0u8); false });
    WHEEL_ROTATIONS_WATCH.dyn_sender().send_if_modified(|value| { *value = Some(0u8); false });
    ODOMETER_WATCH.dyn_sender().send_if_modified(|value| { *value = Some(0u16); false });

    BATTERY_CURRENT_WATCH.dyn_sender().send_if_modified(|value| { *value = Some(0u16); false });
    BATTERY_VOLTAGE_WATCH.dyn_sender().send_if_modified(|value| { *value = Some(0u16); false });
    BATTERY_POWER_WATCH.dyn_sender().send_if_modified(|value| { *value = Some(0u16); false });
    BATTERY_LEVEL_WATCH.dyn_sender().send_if_modified(|value| { *value = Some(0u8); false });
    BATTERY_IN_WATCH.dyn_sender().send_if_modified(|value| { *value = Some([0u16, 0u16]); false });
    
    LED_MODE_WATCH.dyn_sender().send_if_modified(|value| { *value = Some(LedModeType::None); false });
    PIEZO_MODE_WATCH.dyn_sender().send_if_modified(|value| { *value = Some(PiezoModeType::None); false });
    
    ALARM_ENABLED_WATCH.dyn_sender().send_if_modified(|value| { *value = Some(false); false });
    ALARM_ALERT_ACTIVE_WATCH.dyn_sender().send_if_modified(|value| { *value = Some(false); false });
    ALARM_MOTION_DETECTED_WATCH.dyn_sender().send_if_modified(|value| { *value = Some(false); false });
    
    UART_WRITE_WATCH.dyn_sender().send_if_modified(|value| { *value = Some(String::new()); false });
    UART_READ_WATCH.dyn_sender().send_if_modified(|value| { *value = Some(String::new()); false });

    STORE_WRITE_WATCH.dyn_sender().send_if_modified(|value| { *value = Some(false); false });
    STORE_UPDATED_WATCH.dyn_sender().send_if_modified(|value| { *value = Some(false); false });
}


// == WATCHES ===
// Watches can not have history
// Return None if max receivers is reached
pub static BRAKE_ON_WATCH: Watch<WatchMutex, bool, 9> = Watch::new();
pub static PARK_BRAKE_ON_WATCH: Watch<WatchMutex, bool, 5> = Watch::new();
pub static CRUISE_LEVEL_WATCH: Watch<WatchMutex, u8, 3> = Watch::new();
pub static CLOCK_HOURS_WATCH: Watch<WatchMutex, u8, 1> = Watch::new();
pub static CLOCK_MINUTES_WATCH: Watch<WatchMutex, u8, 1> = Watch::new();
pub static THROTTLE_IN_WATCH: Watch<WatchMutex, u16, 3> = Watch::new();
pub static THROTTLE_OUT_WATCH: Watch<WatchMutex, u16, 1> = Watch::new();
pub static TEMPERATURE_WATCH: Watch<WatchMutex, u8, 1> = Watch::new();
pub static POWER_ON_WATCH: Watch<WatchMutex, bool, 9> = Watch::new();
pub static SWITCH_HORN_WATCH: Watch<WatchMutex, bool, 1> = Watch::new();
pub static SWITCH_LIGHT_WATCH: Watch<WatchMutex, bool, 1> = Watch::new();
pub static INSTANT_SPEED_WATCH: Watch<WatchMutex, u32, 1> = Watch::new();
pub static SMOOTH_SPEED_WATCH: Watch<WatchMutex, u8, 1> = Watch::new();
pub static WHEEL_ROTATIONS_WATCH: Watch<WatchMutex, u8, 1> = Watch::new();
pub static ODOMETER_WATCH: Watch<WatchMutex, u16, 1> = Watch::new();

pub static BATTERY_CURRENT_WATCH: Watch<WatchMutex, u16, 1> = Watch::new();
pub static BATTERY_VOLTAGE_WATCH: Watch<WatchMutex, u16, 1> = Watch::new();
pub static BATTERY_POWER_WATCH: Watch<WatchMutex, u16, 1> = Watch::new();
pub static BATTERY_LEVEL_WATCH: Watch<WatchMutex, u8, 1> = Watch::new();
pub static BATTERY_IN_WATCH: Watch<WatchMutex, [u16; 2], 1> = Watch::new();

pub type LedModeType = crate::tasks::led::LedMode;
pub static LED_MODE_WATCH: Watch<WatchMutex, LedModeType, 1> = Watch::new();

pub type PiezoModeType = crate::tasks::piezo::PiezoMode;
pub static PIEZO_MODE_WATCH: Watch<WatchMutex, PiezoModeType, 1> = Watch::new();

pub static ALARM_ENABLED_WATCH: Watch<WatchMutex, bool, 3> = Watch::new();
pub static ALARM_ALERT_ACTIVE_WATCH: Watch<WatchMutex, bool, 1> = Watch::new();
pub static ALARM_MOTION_DETECTED_WATCH: Watch<WatchMutex, bool, 1> = Watch::new();

const MAX_LENGTH: usize = 32;
pub static UART_WRITE_WATCH: Watch<WatchMutex, String<MAX_LENGTH>, 1> = Watch::new();
pub static UART_READ_WATCH: Watch<WatchMutex, String<MAX_LENGTH>, 1> = Watch::new();

pub static STORE_WRITE_WATCH: Watch<WatchMutex, bool, 1> = Watch::new();
pub static STORE_UPDATED_WATCH: Watch<WatchMutex, bool, 1> = Watch::new();



// == CHANNELS ==
// Channels can have history
// <Mutex Type, Data Type, Max Channels(History), Max Subscribers, Max Publishers>
#[derive(Clone, Copy, Ord, PartialOrd, PartialEq, Eq)]
pub struct BleCommandQueue<'a> {
    pub priority: u8,
    pub handle: u16,
    pub message: &'a [u8],
}
pub static BLE_QUEUE_CHANNEL: PriorityChannel::<ChannelMutex, BleCommandQueue, Max, 3> = PriorityChannel::new();

/*
pub static EXAMPLE: PubSubChannel<ChannelMutex, bool, 1, 2, 2> = PubSubChannel::new();
 */

// == MUTEX'S ==
// Mutex dont notify, only for use in loops

pub static CRUISE_VOLTAGE_MUTEX: Mutex<SignalMutex, u16> = Mutex::new(0u16);
pub static CRUISE_VOLTAGES_MUTEX: Mutex<SignalMutex, [u16;5]> = Mutex::new([
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
pub static ALARM_ACTIVE_MUTEX: Mutex<SignalMutex, bool> = Mutex::new(false);


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
pub static THROTTLE_SETTINGS_MUTEX: Mutex<SignalMutex, ThrottleSettings> = Mutex::new(ThrottleSettings {
    passthrough: false, // disable smoothing and limiting
    increase_smooth_factor: 75, // rate of smoothing to acceleration
    decrease_smooth_factor: 150, // rate of smoothing to deceleration
    throttle_min: 900, // mv no throttle
    throttle_max: 3600, // mv full throttle
    deadband_min: 1200, // mv just before motor active
    deadband_max: 2000, // mv just after max speed, or supply voltage
    speed_limit: 4000, // as mv
});