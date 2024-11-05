use embassy_embedded_hal::shared_bus::blocking::i2c::I2cDevice;
use embassy_nrf::{peripherals::TWISPI0, twim::Twim};
use defmt::info;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::blocking_mutex::Mutex;
use core::cell::RefCell;
use embedded_hal::i2c::I2c;

const TASK_ID: &str = "I2C SCAN";

#[embassy_executor::task]
pub async fn task (
    i2c_bus: &'static Mutex<NoopRawMutex, RefCell<Twim<'static, TWISPI0>>>
) {
    info!("{}", TASK_ID);
    let mut i2c = I2cDevice::new(i2c_bus);
    let mut count = 0;
    for address in 1..128 {
        let result = i2c.write(address, &[]);
        match result {
            Ok(_) => {
                info!("{}: device: 0x{:X}", TASK_ID, address);
                count +=1;
            }
            Err(_) => continue,
        }
    }
    info!("{}: found {} devices", TASK_ID, count);
}