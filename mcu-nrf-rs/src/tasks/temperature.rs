use crate::utils::signals;
use embassy_embedded_hal::shared_bus::asynch::i2c::I2cDevice;
use embassy_nrf::{peripherals::TWISPI0, twim::Twim};
use defmt::info;
use mpu6050_async::Mpu6050;
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::mutex;
use embassy_time::{Delay, Timer};
use embassy_futures::select::{select, Either};

const TASK_ID : &str = "TEMPERATURE";
const INTERVAL: u64 = 10; // seconds

#[embassy_executor::task]
pub async fn task(
    i2c_bus: &'static mutex::Mutex<ThreadModeRawMutex, Twim<'static, TWISPI0>>
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


async fn run(i2c_bus: &'static mutex::Mutex<ThreadModeRawMutex, Twim<'static, TWISPI0>>) {
    let i2c = I2cDevice::new(i2c_bus);
    let mut mpu = Mpu6050::new(i2c);
    match mpu.init(&mut Delay).await {
        Ok(()) => {},
        Err(_e) => {
            info!("{}: device error", TASK_ID);
            return;
        }, // unable to communicate with device
    }

    let mut last_temperature: u8 = 0;

    loop {
        // profile in 0ms
        match mpu.get_temp().await {
            Ok(t) => {
                let temp = t as u8;
                if last_temperature != temp {
                    last_temperature = temp;
                    //info!("{}: {}", TASK_ID, temp);
                    signals::send_ble(signals::BleHandles::Temperature, temp.to_le_bytes().as_slice());
                }
            },
            Err(_e) => { info!("{}: device error", TASK_ID); },
        }

        Timer::after_secs(INTERVAL).await;
    }
}