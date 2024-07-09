use crate::signals;
use embassy_embedded_hal::shared_bus::blocking::i2c::I2cDevice;
use embassy_nrf::{peripherals::TWISPI0, twim::Twim};
use defmt::*;
use ads1x1x::{Ads1x1x, ChannelSelection, DataRate16Bit, DynamicOneShot, FullScaleRange, SlaveAddr};
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_time::Timer;
use nb::block;

const TASK_ID: &str = "SAADC";

#[embassy_executor::task]
pub async fn saadc (
    pin: AnyPin
) {
    info!("{}: start", TASK_ID);
    /*
    For NRF52840
    Analog pin  GPIO pin
    AIN0        P0.02
    AIN1        P0.03
    AIN2        P0.04
    AIN3        P0.05
    AIN4        P0.28
    AIN5        P0.29
    AIN6        P0.30
    AIN7        P0.31
    */

    // test saadc with fixed voltage
    embassy_nrf::interrupt::typelevel::SAADC::set_priority(Priority::P2);
    bind_interrupts!(struct Irqs {
        SAADC => saadc::InterruptHandler;
    });
    let config = Config::default();
    let channel_config = ChannelConfig::single_ended(p.P0_02);
    let mut saadc = Saadc::new(p.SAADC, Irqs, config, [channel_config]);
    saadc.calibrate().await;
    Timer::after_millis(500).await;

    loop {
        let mut buf = [0; 1];
        saadc.sample(&mut buf).await;
        let input = buf[0];
        let voltage = f32::from(input) * 3600.0 / 4096.0; // converted to mv
        info!("sample: {}", voltage);
        Timer::after_millis(100).await;
    }

}