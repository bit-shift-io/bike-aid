use core::future;
use crate::utils::{i2c, signals};
use embassy_embedded_hal::shared_bus::asynch::i2c::I2cDevice;
use embassy_nrf::{peripherals::TWISPI0, twim::Twim};
use defmt::info;
use embassy_futures::select::{select3, Either3};
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::mutex;
use embedded_ads111x::{ADS111x, ADS111xConfig, DataRate, InputMultiplexer, ProgramableGainAmplifier};
use embassy_time::Timer;

const TASK_ID : &str = "THROTTLE ADC";
const INTERVAL: u64 = 98; // less 2ms for read time
const ADDRESS: u8 = 0x48;

#[embassy_executor::task]
pub async fn task(
    i2c_bus: &'static mutex::Mutex<ThreadModeRawMutex, Twim<'static, TWISPI0>>
) {
    info!("{}", TASK_ID);

    let mut rec_power_on = signals::POWER_ON.receiver().unwrap();
    let mut state_power_on = rec_power_on.try_get().unwrap();

    let mut rec_park_brake_on = signals::PARK_BRAKE_ON.receiver().unwrap();
    let mut state_park_brake_on = rec_park_brake_on.try_get().unwrap();

    loop {
        match select3(rec_power_on.changed(), rec_park_brake_on.changed(), run(state_power_on, state_park_brake_on, i2c_bus)).await {
            Either3::First(b) => { state_power_on = b; },
            Either3::Second(b) => { state_park_brake_on = b;},
            Either3::Third(_) => {}
        }
    }
}


pub async fn run(power_on: bool, park_brake_on: bool, i2c_bus: &'static mutex::Mutex<ThreadModeRawMutex, Twim<'static, TWISPI0>>) {
    if power_on && !park_brake_on { throttle_adc(i2c_bus).await }
    future::pending().await // wait/yield forever doing nothing
}


async fn throttle_adc(i2c_bus: &'static mutex::Mutex<ThreadModeRawMutex, Twim<'static, TWISPI0>>) {
    // check if device available
    if !i2c::device_available(i2c_bus, ADDRESS).await {
        info!("{}: end", TASK_ID);
        return;
    }

    // init device
    let i2c = I2cDevice::new(i2c_bus);
    let config = ADS111xConfig::default()
        .mux(InputMultiplexer::AIN0GND)
        .dr(DataRate::SPS860) // higher data rate completes read in 2ms instead of 120ms
        .pga(ProgramableGainAmplifier::V6_144); // 6.144v

    let mut adc = match ADS111x::new(i2c, ADDRESS, config) { // 0x48
        Err(_e) => {
            info!("{}: device error", TASK_ID);
            return;
        },
        Ok(x) => x, // assign the mutex to adc
    };

    // Write the configuration to the chip's registers
    if let Err(_e) = adc.write_config(None).await {
        info!("{}: device error", TASK_ID);
        return;
    };

    let send_throttle = signals::THROTTLE_IN.sender();

    loop {
        Timer::after_millis(INTERVAL).await;
        
        // read takes 2ms | 83 ticks
        match adc.read_single_voltage(None).await {
            Ok(v) => {
                // convert to voltage -> 6.144v * 1000 (to mv) / 32768 (15 bit, 1 bit +-)
                //let input_voltage: u16 = (v * 6144.0 / 32768.0) as u16; // mv
                let input_voltage = (v * 1000f32) as u16; // mv
        
                if input_voltage > 20 {
                    send_throttle.send(input_voltage);
                };
            },
            Err(_e) => info!("{}: device error", TASK_ID),
        }
    }
}