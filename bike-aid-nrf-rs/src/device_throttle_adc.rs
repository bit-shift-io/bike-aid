use crate::signals;
use embassy_embedded_hal::shared_bus::blocking::i2c::I2cDevice;
use embassy_nrf::{peripherals::TWISPI0, twim::Twim};
use defmt::*;
use ads1x1x::{Ads1x1x, ChannelSelection, DynamicOneShot, SlaveAddr, FullScaleRange};
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_time::Timer;
use nb::block;

static DEVICE_ID : &str = "THROTTLE ADC";

#[embassy_executor::task]
pub async fn adc (
    i2c: I2cDevice<'static,NoopRawMutex, Twim<'static,TWISPI0>>
) {
    let pub_throttle = signals::THROTTLE_IN.publisher().unwrap();
    let address = SlaveAddr::default(); // 0x48
    let mut adc = Ads1x1x::new_ads1115(i2c, address);
    let _ = adc.set_full_scale_range(FullScaleRange::Within6_144V); // +-6.144v
    // 6.144 / 32768 = 0.0001875V (15 bit)

    info!("{} : Entering main loop", DEVICE_ID);
    loop {
        let value = block!(adc.read(ChannelSelection::SingleA0)).unwrap();
        pub_throttle.publish_immediate(value);
        Timer::after_millis(200).await;
    }
}