use crate::signals;
use embassy_embedded_hal::shared_bus::blocking::i2c::I2cDevice;
use embassy_nrf::{peripherals::TWISPI0, twim::Twim};
use defmt::*;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_time::Timer;
use mcp4725::MCP4725;

static DEVICE_ID : &str = "THROTTLE DAC";

#[embassy_executor::task]
pub async fn dac (
    i2c: I2cDevice<'static,NoopRawMutex, Twim<'static,TWISPI0>>
) {
    let address = 0x60;
    let mut sub_throttle = signals::THROTTLE.subscriber().unwrap();
    let mut dac = MCP4725::new(i2c, address);
    let _ = dac.set_dac_and_eeprom(mcp4725::PowerDown::Normal, 0); // set 0 volts output

    info!("{} : Entering main loop", DEVICE_ID);
    loop {
        //let val = sub_throttle.next_message_pure().await;
        //let _ = dac.set_dac(mcp4725::PowerDown::Normal, val as u16);
        let read = dac.read().unwrap();
        info!("DAC: {}", read.data());
        Timer::after_millis(200).await;
    }
}