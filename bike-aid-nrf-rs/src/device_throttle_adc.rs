use crate::signals;
use embassy_embedded_hal::shared_bus::blocking::i2c::I2cDevice;
use embassy_nrf::{peripherals::TWISPI0, twim::Twim};
use defmt::*;
use ads1x1x::{Ads1x1x, ChannelSelection, DynamicOneShot, SlaveAddr};
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_time::Timer;
use nb::block;

static DEVICE_ID : &str = "THROTTLE ADC";

#[embassy_executor::task]
pub async fn adc (
    i2c: I2cDevice<'static,NoopRawMutex, Twim<'static,TWISPI0>>
) {
    let mut pub_throttle = signals::THROTTLE.subscriber().unwrap();
    let address = SlaveAddr::default(); // 0x48
    let mut adc = Ads1x1x::new_ads1115(i2c, address);

    info!("{} : Entering main loop", DEVICE_ID);
    loop {
        let measurement = block!(adc.read(ChannelSelection::SingleA0)).unwrap();
        //let val = adc.read(ChannelSelection::SingleA0).unwrap();
        info!("ADC:{}", measurement);
        Timer::after_millis(200).await;
    }
}