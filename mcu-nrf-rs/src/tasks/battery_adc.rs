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
const INTERVAL: u64 = 1000;

// consts for voltage divider
const VOLTAGE_MULTIPLIER : f32 = (0_995_700.0 + 51_270.0) / 51_270.0; // ((R1 + R2) / R2)

// consts for ACS758LCB-100B
const VCC : f32 = 3.3; // 3.3v
const QUIESCENT_OUTPUT_VOLTAGE : f32 = 0.5; // for ACS758LCB-100B
const FACTOR: f32 = 20.0/1000.0; // 20.0 for ACS758LCB-100B
const QOV: f32 = QUIESCENT_OUTPUT_VOLTAGE * VCC; // set quiescent Output voltage
const CUTOFF_LIMIT: f32 = 2.0; // for model use 2
const CUTOFF: f32 = FACTOR / CUTOFF_LIMIT; // convert current cut off to mV
const NON_ZERO: f32 = 0.007; // value to make voltage zero when there is no current


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
    let address = SlaveAddr::Alternative(false, true); // sda 0x4A
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
        let value_a0 = block!(adc.read(ChannelSelection::SingleA0)).unwrap();
        let value_a1 = block!(adc.read(ChannelSelection::SingleA1)).unwrap();

        // convert to voltage
        // ADC - 4.096v * 1000 (to mv) / 32768 (15 bit, 1 bit +-)
        let input_voltage_a0: i16 = (f32::from(value_a0) * 4096.0 / 32768.0) as i16; // converted to mv
        let input_voltage_a1: i16 = (f32::from(value_a1) * 4096.0 / 32768.0) as i16; // converted to mv

        // voltage before the resitor divider
        // vIn = vOut * ((R1 + R2) / R2)
        let real_voltage = (f32::from(input_voltage_a1) * VOLTAGE_MULTIPLIER) as i16;

        // current sensor
        let current_voltage = f32::from(input_voltage_a0) - QOV + NON_ZERO;
        let mut current = current_voltage / FACTOR;
        if abs(current) < CUTOFF {
            current = 0.0;
        }
        let real_current = (current * 1000.0) as i16; // convert to mA


        // Note, the impedance acts as a 10mo resistor from pin to ground, so need to calulate that also!?

        // AO is current in mA
        // A1 is voltage in mV
        pub_data.publish_immediate([real_current, real_voltage]);
        Timer::after_millis(INTERVAL).await;
    }
}