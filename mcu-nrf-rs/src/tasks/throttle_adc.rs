use crate::utils::signals;
use embassy_embedded_hal::shared_bus::blocking::i2c::I2cDevice;
use embassy_nrf::{peripherals::TWISPI0, twim::Twim};
use defmt::*;
use embassy_futures::select::{select, Either};
use ads1x1x::{Ads1x1x, ChannelSelection, DynamicOneShot, FullScaleRange, SlaveAddr};
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::blocking_mutex::Mutex;
use core::cell::RefCell;
use embassy_time::Timer;
use nb::block;

const TASK_ID : &str = "THROTTLE ADC";
const INTERVAL: u64 = 100;

#[embassy_executor::task]
pub async fn task(
    i2c_bus: &'static Mutex<NoopRawMutex, RefCell<Twim<'static, TWISPI0>>>
) {
    info!("{}", TASK_ID);

    // power on/off
    let mut sub = signals::SWITCH_POWER.subscriber().unwrap();
    let mut state = false;

    loop { 
        if let Some(b) = sub.try_next_message_pure() {state = b}
        match state {
            true => {
                let sub_future = sub.next_message_pure();
                let task_future = park_brake(i2c_bus);
                match select(sub_future, task_future).await {
                    Either::First(val) => { state = val; }
                    Either::Second(_) => { Timer::after_secs(60).await; } // retry
                }
            },
            false => { state = sub.next_message_pure().await; }
        }
    }
}


async fn park_brake(i2c_bus: &'static Mutex<NoopRawMutex, RefCell<Twim<'static, TWISPI0>>>) {
    // park brake on/off
    let mut watch = signals::PARK_BRAKE_ON_WATCH.receiver().unwrap();
    let mut state = true; // default to on

    loop { 
        if let Some(b) = watch.try_get() {state = b}
        match state {
            false => {
                let watch_future = watch.changed();
                let task_future = run(i2c_bus);
                match select(watch_future, task_future).await {
                    Either::First(val) => { state = val; }
                    Either::Second(_) => { Timer::after_secs(60).await; } // retry
                }
            },
            true => { state = watch.changed().await; }
        }
    }
}


async fn run(i2c_bus: &'static Mutex<NoopRawMutex, RefCell<Twim<'static, TWISPI0>>>) {
    let i2c = I2cDevice::new(i2c_bus);
    let send_throttle = signals::THROTTLE_IN_WATCH.sender();
    let address = SlaveAddr::default(); // 0x48
    let mut adc = Ads1x1x::new_ads1115(i2c, address);
    let result = adc.set_full_scale_range(FullScaleRange::Within6_144V); // Within2_048V +- 2.048v // Within6_144V +-6.144v
    match result {
        Ok(()) => {},
        Err(_e) => {
            info!("{}: device error", TASK_ID);
            return
        }, // unable to communicate with device
    }
    
    loop {
        Timer::after_millis(INTERVAL).await;

        //let value = adc.read(ChannelSelection::SingleA0).unwrap(); // crash here
        let value = block!(adc.read(ChannelSelection::SingleA0)).unwrap();

        // convert to voltage
        // ADC - 6.144v * 1000 (to mv) / 32768 (15 bit, 1 bit +-)
        let input_voltage: u16 = (f32::from(value) * 6144.0 / 32768.0) as u16; // converted to mv


        // voltage of the actual throttle before the resitor divider
        // some minor inaccuracy here from resitors, is it worth compensating for 2-3mv?
        //let real_voltage = (input_voltage * 5 / 2) as u16; // 2 resitor values 330 & 220 : 5v = 2v

        // Note, the impedance acts as a 10mo resistor from pin to ground, so need to calulate that also!
        // note ive added 100k pulldown resistor to remove fluctation during power off
        // so do a check, if value is larger than 20 we can report it
        if input_voltage > 20 {
            send_throttle.send(input_voltage);
        }
    }
}