use crate::utils::signals;
use embassy_embedded_hal::shared_bus::blocking::i2c::I2cDevice;
use embassy_nrf::{peripherals::TWISPI0, twim::Twim};
use defmt::*;
use embassy_futures::select::{select, Either};
use ads1x1x::{Ads1x1x, ChannelSelection, DynamicOneShot, FullScaleRange, SlaveAddr};
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::blocking_mutex::Mutex;
use num_traits::abs;
use core::cell::RefCell;
use embassy_time::Timer;
use nb::block;

const TASK_ID: &str = "BATTERY ADC";
const INTERVAL: u64 = 30; // 30 seconds

// consts for voltage divider
const VOLTAGE_CALIBATION : u16 = 10; // calibration level = multimeter - measured
const R_CALIBRATION : f32 = 0.050; // adjust resistor divider calibration
const R1 : f32 = 1_000_000.0; // 0_995_700.0
const R2 : f32 = 51_000.0; // 51_270.0
const VOLTAGE_MULTIPLIER : f32 = ((R1 + R2) / R2) - R_CALIBRATION; // ((R1 + R2) / R2)

// consts for ACS758LCB-100B
const VCC : f32 = 3300.0; // 3.3v = 3,300mV
const QUIESCENT_OUTPUT_VOLTAGE : f32 = 500.0; // 0.5v for ACS758LCB-100B = 500mV
const FACTOR: f32 = 20.0/1000.0; // 20.0 for ACS758LCB-100B
const QOV: f32 = QUIESCENT_OUTPUT_VOLTAGE * VCC; // set quiescent Output voltage
const CUTOFF_LIMIT: f32 = 2.0; // for model use 2A
const CUTOFF: f32 = FACTOR / CUTOFF_LIMIT; // convert current cut off to mV
const NON_ZERO: f32 = 7.0; // 0.007v = 7mV value to make voltage zero when there is no current


#[embassy_executor::task]
pub async fn task(
    i2c_bus: &'static Mutex<NoopRawMutex, RefCell<Twim<'static, TWISPI0>>>
) {
    info!("{}: start", TASK_ID);

    let mut sub_power = signals::SWITCH_POWER.subscriber().unwrap();
    let mut power_state = false;

    loop { 
        if let Some(b) = sub_power.try_next_message_pure() {power_state = b}
        match power_state {
            true => {
                let power_future = sub_power.next_message_pure();
                let task_future = run(i2c_bus);
                match select(power_future, task_future).await {
                    Either::First(val) => { power_state = val; }
                    Either::Second(_) => { Timer::after_secs(60).await; } // retry
                }
            },
            false => { 
                // TODO: when power off, we still want to get voltage once an hour or so
                power_state = sub_power.next_message_pure().await; 
            }
        }
    }
}


async fn run(i2c_bus: &'static Mutex<NoopRawMutex, RefCell<Twim<'static, TWISPI0>>>) {
    let i2c = I2cDevice::new(i2c_bus);
    let pub_data = signals::BATTERY_IN.publisher().unwrap();
    let address = SlaveAddr::Alternative(true, false); // new_sda(); //// sda 0x4A
    let mut adc = Ads1x1x::new_ads1115(i2c, address);

    let result = adc.set_full_scale_range(FullScaleRange::Within4_096V); // set range to 4.096v
    match result {
        Ok(()) => {},
        Err(_e) => {
            info!("{}: device error", TASK_ID);
            return
        }, // unable to communicate with device
    }

    loop {
        let value_a0 = block!(adc.read(ChannelSelection::SingleA0)).unwrap(); // current
        let value_a1 = block!(adc.read(ChannelSelection::SingleA1)).unwrap(); // voltage

        let voltage = calculate_voltage(value_a1);
        let current = calculate_current(value_a0);

        pub_data.publish_immediate([voltage, current]);
        Timer::after_secs(INTERVAL).await;
    }
}


fn calculate_voltage(voltage: i16) -> u16 {
    // convert to voltage
    // ADC - 4.096v * 1000 (to mv) / 32768 (15 bit, 1 bit +-)
    let mut input_voltage_a1: u16 = (f32::from(voltage) * 4096.0 / 32768.0) as u16; // converted to mv

    // calibration
    input_voltage_a1 += VOLTAGE_CALIBATION; 

    //info!("{}: a0: {} -> {}, a1: {} -> {}", TASK_ID, value_a0, input_voltage_a0, value_a1, input_voltage_a1);
    //info!("{}: multiplier: {}", TASK_ID, VOLTAGE_MULTIPLIER);
    
    // voltage before the resitor divider
    (f32::from(input_voltage_a1) * VOLTAGE_MULTIPLIER) as u16 // mv
}


fn calculate_current(current: i16) -> u16 {
    // convert to voltage
    // ADC - 4.096v * 1000 (to mv) / 32768 (15 bit, 1 bit +-)
    let input_voltage_a0: u16 = (f32::from(current) * 4096.0 / 32768.0) as u16; // converted to mv

    // TODO: current sensor
    let current_voltage = f32::from(input_voltage_a0) - QOV + NON_ZERO;
    let mut current = current_voltage / FACTOR;
    if abs(current) < CUTOFF { // cutoff in mA
        current = 0.0;
    }
    
    (current * 1000.0) as u16 // convert to mA
}