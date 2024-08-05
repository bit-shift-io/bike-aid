use crate::utils::signals;
use embassy_embedded_hal::shared_bus::blocking::i2c::I2cDevice;
use embassy_nrf::{peripherals::TWISPI0, twim::Twim};
use defmt::*;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_time::{Delay, Timer};
use mpu6050::*;

const TASK_ID : &str = "TEMPERATURE";
const INTERVAL: u64 = 20;

#[embassy_executor::task]
pub async fn temperature (
    i2c: I2cDevice<'static,NoopRawMutex, Twim<'static,TWISPI0>>
) {
    info!("{}: start", TASK_ID);

    let mut mpu = Mpu6050::new(i2c);
    let mut delay = Delay;
    let result = mpu.init(&mut delay);
    match result {
        Ok(()) => {},
        Err(_e) => {
            info!("{} : device error", TASK_ID);
            return
        }, // unable to communicate with device
    }

    let pub_temperature = signals::TEMPERATURE.publisher().unwrap();

    loop {
        let temp = mpu.get_temp().unwrap();
        info!("{}: {}", TASK_ID, temp);
        pub_temperature.publish_immediate(temp as u8); // in degrees C, no decimals
        Timer::after_secs(INTERVAL).await;
    }
}