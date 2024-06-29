use crate::signals;
use embassy_nrf::twim::Twim;
use embassy_time::{Duration, Timer};
use defmt::*;
use mcp4725::{MCP4725};

static DEVICE_ID : &str = "DAC";

#[embassy_executor::task]
pub async fn dac (
    twi : Twim<'static, embassy_nrf::peripherals::TWISPI0>, // todo , make generic?
    address : u8
) {
    //let sub_write_value = signals::THROTTLE_OUTPUT.publisher().unwrap();
    let mut dac = MCP4725::new(twi, address);
    let _ = dac.set_dac(mcp4725::PowerDown::Normal, 0); // set 0 volts output

    info!("{} : Entering main loop", DEVICE_ID);
    loop {
        let val = dac.read().unwrap();
        info!("{}", val.data());
        let data = 1024;
        let _ = dac.set_dac(mcp4725::PowerDown::Normal, data);
        Timer::after(Duration::from_millis(100)).await;
    }
}