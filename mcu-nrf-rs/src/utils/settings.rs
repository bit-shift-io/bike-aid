use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, mutex::Mutex, watch::Watch};

// configure types
type SettingsMutex = ThreadModeRawMutex;
type WatchMutex = ThreadModeRawMutex;


pub static CRUISE_VOLTAGE: Watch<WatchMutex, u16, 1> = Watch::new_with(0u16);
pub static CRUISE_VOLTAGES: Mutex<SettingsMutex, [u16;5]> = Mutex::new([
        1600u16, // 1408 a little too slow, 1500 a little slow
        2000u16, // 1906 a little to slow
        2300u16, // 2400 a little fast?
        2800u16, // 2900 ?
        3400u16, // 3400 max
    ]);


#[derive(Clone, Copy)]
pub struct AlarmSettings {
    pub acc_sensitivity: f32,
    pub gyro_sensitivity: f32,
    pub angle_sensitivity: f32,
    pub warning_count: u8,
}


pub static ALARM_SETTINGS: Watch<WatchMutex, AlarmSettings, 2> = Watch::new_with(AlarmSettings {
    acc_sensitivity: 0.9,
    gyro_sensitivity: 0.8,
    angle_sensitivity: 0.1,
    warning_count: 3
});


/* 
Deadband / Deadzone
===========================
Adjust throttle range to eliminate deadband/deadzones. 
All the ranges below can be determined by watching the serial console and twisting the throttle.
Or use a multimeter to measure voltage output from the throttle on your ebike.

throttle_min - Voltage when the throttle is unpressed
throttle_max - Voltage when the throttle is fully pressed
deadband_min - Voltage just before the motor starts to activate the wheels
deadband_max - Voltage just after max speed (or use supply voltage otherwise)

Smoothing - Jerkiness Mitigation
===========================
How quickly to adjust output over time.
Larger values are faster, lower values are slower.

Speed Limit
===========================
Adjusts throttle output speed limit.
*/

#[derive(Clone, Copy)]
pub struct ThrottleSettings {
    pub passthrough: bool,
    pub increase_smoothing_low: u16,
    pub increase_smoothing_high: u16,
    pub decrease_smoothing: u16,
    pub throttle_min: u16,
    pub throttle_max: u16,
    pub deadband_min: u16,
    pub deadband_max: u16,
    pub speed_step: u16,
}


/*
Controller supply voltage - 4.36v = 4360mv
*/
pub static THROTTLE_SETTINGS: Watch<WatchMutex, ThrottleSettings, 2> = Watch::new_with(ThrottleSettings {
    passthrough: false, // disable smoothing and limiting
    increase_smoothing_low: 80, // rate of smoothing to acceleration at the low end of the throttle
    increase_smoothing_high: 30, // rate of smoothing to acceleration at the high end of the throttle
    decrease_smoothing: 80, // rate of smoothing to deceleration
    throttle_min: 900, // mv no throttle
    throttle_max: 3600, // mv full throttle
    deadband_min: 1200, // mv just before motor active
    deadband_max: 2000, // mv just after max speed, or supply voltage
    speed_step: 1600, // jump/step to this voltage when throttle engages
});