use crate::utils::signals;
use embassy_embedded_hal::shared_bus::blocking::i2c::I2cDevice;
use embassy_nrf::{peripherals::TWISPI0, twim::Twim};
use defmt::*;
use ads1x1x::{Ads1x1x, ChannelSelection, DynamicOneShot, FullScaleRange, SlaveAddr};
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::blocking_mutex::Mutex;
use core::cell::RefCell;
use embassy_time::Timer;
use nb::block;

const TASK_ID: &str = "Battery ADC";
const INTERVAL: u64 = 1000;

#[embassy_executor::task]
pub async fn task(
    i2c_bus: &'static Mutex<NoopRawMutex, RefCell<Twim<'static, TWISPI0>>>
    //i2c: I2cDevice<'static,NoopRawMutex, Twim<'static,TWISPI0>>
) {
    return; // TODO: fix address
    info!("{}: start", TASK_ID);

    let i2c = I2cDevice::new(i2c_bus);
    let pub_data = signals::BATTERY_IN.publisher().unwrap();

    let address = SlaveAddr::Alternative(true, true); // default used for throttle 0x48
    let mut adc = Ads1x1x::new_ads1115(i2c, address);
    let result = adc.set_full_scale_range(FullScaleRange::Within6_144V); // Within2_048V +- 2.048v // Within6_144V +-6.144v
    match result {
        Ok(()) => {},
        Err(_e) => {
            info!("{} : device error", TASK_ID);
            return
        }, // unable to communicate with device
    }
    //let _ = adc.set_data_rate(DataRate16Bit::Sps8);

    loop {
        //let value = adc.read(ChannelSelection::SingleA0).unwrap(); // crash here
        let value_a0 = block!(adc.read(ChannelSelection::SingleA0)).unwrap();
        let value_a1 = block!(adc.read(ChannelSelection::SingleA1)).unwrap();

        // clamp to positive values only
        //let input = clamp_positive(input);

        // convert to voltage
        // ADC - 6.144v * 1000 (to mv) / 32768 (15 bit, 1 bit +-)
        let input_voltage_a0: i16 = (f32::from(value_a0) * 6144.0 / 32768.0) as i16; // converted to mv
        let input_voltage_a1: i16 = (f32::from(value_a1) * 6144.0 / 32768.0) as i16; // converted to mv


        // voltage of the actual throttle before the resitor divider
        // some minor inaccuracy here from resitors, is it worth compensating for 2-3mv?
        //let real_voltage = (input_voltage * 5 / 2) as u16; // 2 resitor values 330 & 220 : 5v = 2v

        // Note, the impedance acts as a 10mo resistor from pin to ground, so need to calulate that also!
        pub_data.publish_immediate([input_voltage_a0,input_voltage_a1]);
        Timer::after_millis(INTERVAL).await;
    }
}