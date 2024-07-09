use crate::{functions::min, signals};
use embassy_embedded_hal::shared_bus::blocking::i2c::I2cDevice;
use embassy_nrf::{peripherals::TWISPI0, twim::Twim};
use defmt::*;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_time::Timer;
use mcp4725::MCP4725;

const TASK_ID : &str = "THROTTLE DAC";

#[embassy_executor::task]
pub async fn dac (
    i2c: I2cDevice<'static,NoopRawMutex, Twim<'static,TWISPI0>>
) {
    info!("{}: start", TASK_ID);
    let address = 0x60;
    let supply_voltage = 4880; // TODO: mv supply for calibration
    let mut sub_throttle = signals::THROTTLE_OUT.subscriber().unwrap();
    let mut dac = MCP4725::new(i2c, address);
    let result = dac.set_dac_and_eeprom(mcp4725::PowerDown::Normal, 0); // set 0 volts output
    match result {
        Ok(()) => {},
        Err(e) => {
            info!("{} : device error", TASK_ID);
            return}, // unable to communicate with device
    }

    loop {
        /*
        // for testing calibration
        dac.set_dac(mcp4725::PowerDown::Normal, (1000 * 4095.0 / supply_voltage) as u16);    //Set voltage to 1V
        Timer::after_secs(2).await;
        dac.set_dac(mcp4725::PowerDown::Normal, (2000 * 4095.0 / supply_voltage) as u16);    //Set voltage to 2V
        Timer::after_secs(2).await;
        dac.set_dac(mcp4725::PowerDown::Normal, (3000 * 4095.0 / supply_voltage) as u16);    //Set voltage to 3V
        Timer::after_secs(2).await;
        dac.set_dac(mcp4725::PowerDown::Normal, (4000 * 4095.0 / supply_voltage) as u16);    //Set voltage to 4V
        Timer::after_secs(2).await;
        dac.set_dac(mcp4725::PowerDown::Normal, (5000 * 4095.0 / supply_voltage) as u16);              //Set voltage to 5V or (Vcc)
        Timer::after_secs(2).await;
        */

        let value = sub_throttle.next_message_pure().await; // desired mv
        let dac_value = (f32::from(value) * 4095.0 / supply_voltage as f32) as u16;
        let dac_value = min(4095, dac_value);
        let _ = dac.set_dac(mcp4725::PowerDown::Normal, dac_value as u16);
    }
}