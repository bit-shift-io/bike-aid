use crate::signals;
use embassy_embedded_hal::shared_bus::blocking::i2c::I2cDevice;
use embassy_nrf::{peripherals::TWISPI0, twim::Twim};
use defmt::*;
use ads1x1x::{Ads1x1x, ChannelSelection, DataRate16Bit, DynamicOneShot, FullScaleRange, SlaveAddr};
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
    let result = adc.set_full_scale_range(FullScaleRange::Within6_144V); // Within2_048V +- 2.048v // Within6_144V +-6.144v
    match result {
        Ok(()) => {},
        Err(e) => {
            info!("{} : device error", DEVICE_ID);
            return}, // unable to communicate with device
    }
    //let _ = adc.set_data_rate(DataRate16Bit::Sps8);

    info!("{} : Entering main loop", DEVICE_ID);
    loop {
        //let value = adc.read(ChannelSelection::SingleA0).unwrap(); // crash here
        let value = block!(adc.read(ChannelSelection::SingleA0)).unwrap();

        // clamp to positive values only
        //let input = clamp_positive(input);

        // convert to voltage
        // ADC - 6.144v * 1000 (to mv) / 32768 (15 bit, 1 bit +-)
        let input_voltage: i16 = (f32::from(value) * 6144.0 / 32768.0) as i16; // converted to mv


        // voltage of the actual throttle before the resitor divider
        // some minor inaccuracy here from resitors, is it worth compensating for 2-3mv?
        //let real_voltage = (input_voltage * 5 / 2) as u16; // 2 resitor values 330 & 220 : 5v = 2v

        pub_throttle.publish_immediate(input_voltage);
        Timer::after_millis(200).await;
    }
}