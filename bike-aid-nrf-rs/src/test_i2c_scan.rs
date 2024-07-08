use embassy_embedded_hal::shared_bus::blocking::i2c::I2cDevice;
use embassy_nrf::{peripherals::TWISPI0, twim::Twim};
use defmt::*;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embedded_hal::i2c::I2c;

static DEVICE_ID : &str = "I2C SCAN";

#[embassy_executor::task]
pub async fn scan (
    mut i2c: I2cDevice<'static,NoopRawMutex, Twim<'static,TWISPI0>>
) {
    info!("{}: start", DEVICE_ID);
    let mut count = 0;
    for address in 1..128 {
        let result = i2c.write(address, &[]);
        match result {
            Ok(_) => {
                info!("I2C/TWI found device: 0x{:X}", address);
                count +=1;
            }
            Err(_) => continue,
        }
    }
    info!("{}: found {} devices", DEVICE_ID, count);
}