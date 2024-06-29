use crate::signals;
use embassy_nrf::twim::Twim;
use defmt::*;
use mcp4725::MCP4725;

static DEVICE_ID : &str = "DAC";

#[embassy_executor::task]
pub async fn dac (
    twi : Twim<'static, embassy_nrf::peripherals::TWISPI0>, // todo , make generic?
    address : u8
) {
    let mut sub_throttle = signals::THROTTLE.subscriber().unwrap();
    let mut dac = MCP4725::new(twi, address);

    // https://crates.io/crates/mcp4725
    let _ = dac.set_dac_and_eeprom(mcp4725::PowerDown::Normal, 0); // set 0 volts output

    info!("{} : Entering main loop", DEVICE_ID);
    loop {
        let val = sub_throttle.next_message_pure().await;
        let _ = dac.set_dac(mcp4725::PowerDown::Normal, val as u16);
        //let read = dac.read().unwrap();
        //info!("{}", read.data());
    }
}