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
    let mut sub_throttle = signals::THROTTLE_OUT.subscriber().unwrap();
    let mut dac = MCP4725::new(i2c, address);
    let result = dac.set_dac_and_eeprom(mcp4725::PowerDown::Normal, 0); // set 0 volts output
    match result {
        Ok(()) => {},
        Err(e) => {
            info!("{} : device error", DEVICE_ID);
            return}, // unable to communicate with device
    }

    info!("{} : Entering main loop", DEVICE_ID);
    loop {
        let value = sub_throttle.next_message_pure().await;
        info!("dac");
        let result = dac.set_dac(mcp4725::PowerDown::Normal, value as u16);
        info!("dac {}", result);
        //let read = dac.read().unwrap();
        //info!("DAC: {}", read.data());
        Timer::after_millis(200).await;
    }
}