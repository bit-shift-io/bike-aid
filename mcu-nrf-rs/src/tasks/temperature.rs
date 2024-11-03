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
) {
    info!("{}", TASK_ID);

    let mut rec = signals::POWER_ON.receiver().unwrap();
    let mut state = false;

    loop { 
        if let Some(b) = rec.try_get() {state = b}
        match state {
            true => {
                let watch_future = rec.changed();
                let task_future = run(i2c_bus);
                match select(watch_future, task_future).await {
                    Either::First(val) => { state = val; }
                    Either::Second(_) => { Timer::after_secs(60).await; } // retry
                }
            },
            false => { state = rec.changed().await; }
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

    let mut last_temperature: u8 = 0;

    loop {
        let temp = mpu.get_temp().unwrap() as u8;

        if last_temperature != temp {
            last_temperature = temp;
            signals::send_ble(signals::BleHandles::Temperature, temp.to_le_bytes().as_slice()).await;
        }
        
        Timer::after_secs(INTERVAL).await;
    }
}