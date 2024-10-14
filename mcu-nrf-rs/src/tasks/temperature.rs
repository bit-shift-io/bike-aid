use crate::utils::signals;
use embassy_embedded_hal::shared_bus::blocking::i2c::I2cDevice;
use embassy_nrf::{peripherals::TWISPI0, twim::Twim};
use defmt::*;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::blocking_mutex::Mutex;
use core::cell::RefCell;
use embassy_time::{Delay, Timer};
use mpu6050::*;
use embassy_futures::select::{select, Either};

const TASK_ID : &str = "TEMPERATURE";
const INTERVAL: u64 = 20; // seconds

#[embassy_executor::task]
pub async fn task(
    i2c_bus: &'static Mutex<NoopRawMutex, RefCell<Twim<'static, TWISPI0>>>
    //mut i2c: I2cDevice<'static,NoopRawMutex, Twim<'static,TWISPI0>>
) {
    info!("{}", TASK_ID);

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
            false => { power_state = sub_power.next_message_pure().await; }
        }
    }
}


async fn run(i2c_bus: &'static Mutex<NoopRawMutex, RefCell<Twim<'static, TWISPI0>>>) {
    let i2c = I2cDevice::new(i2c_bus);
    let mut mpu = Mpu6050::new(i2c);
    let result = mpu.init(&mut Delay);
    match result {
        Ok(()) => {},
        Err(_e) => {
            info!("{}: device error", TASK_ID);
            return
        }, // unable to communicate with device
    }

    let pub_temperature = signals::TEMPERATURE.publisher().unwrap();

    loop {
        let temp = mpu.get_temp().unwrap();
        pub_temperature.publish_immediate(temp as u8); // in degrees C, no decimals
        Timer::after_secs(INTERVAL).await;
    }
}