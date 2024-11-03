use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, mutex::Mutex};

// configure types
type SignalMutex = ThreadModeRawMutex;

pub static CRUISE_VOLTAGE: Mutex<SignalMutex, u16> = Mutex::new(0u16);
pub static CRUISE_VOLTAGES: Mutex<SignalMutex, [u16;5]> = Mutex::new([
        1600u16, // 1408 a little too slow, 1500 a little slow
        2000u16, // 1906 a little to slow
        2300u16, // 2400 a little fast?
        2800u16, // 2900 ?
        3400u16, // 3400 max
    ]);


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