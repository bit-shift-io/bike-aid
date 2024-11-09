use crate::utils::{i2c, signals};
use embassy_embedded_hal::shared_bus::asynch::i2c::I2cDevice;
use embassy_nrf::{peripherals::TWISPI0, twim::Twim};
use defmt::info;
use embassy_futures::select::{select, Either};
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

    if !i2c::device_available(i2c_bus, ADDRESS).await {
        info!("{}: end", TASK_ID);
        return;
    }
    
    // power on/off
    let mut rec = signals::POWER_ON.receiver().unwrap();
    let mut state = false;

    loop { 
        if let Some(b) = rec.try_get() {state = b}
        match state {
            true => {
                let watch_future = rec.changed();
                let task_future = park_brake(i2c_bus);
                match select(watch_future, task_future).await {
                    Either::First(val) => { state = val; }
                    Either::Second(_) => { Timer::after_secs(60).await; } // retry
                }
            },
            false => { state = rec.changed().await; }
        }
    }
}


async fn park_brake(i2c_bus: &'static mutex::Mutex<ThreadModeRawMutex, Twim<'static, TWISPI0>>) {
    // park brake on/off
    let mut watch = signals::PARK_BRAKE_ON.receiver().unwrap();
    let mut state = true; // default to on

    loop { 
        if let Some(b) = watch.try_get() {state = b}
        match state {
            false => {
                let watch_future = watch.changed();
                let task_future = run(i2c_bus);
                match select(watch_future, task_future).await {
                    Either::First(val) => { state = val; }
                    Either::Second(_) => { Timer::after_secs(60).await; } // retry
                }
            },
            true => { state = watch.changed().await; }
        }
    }
}


async fn run(i2c_bus: &'static mutex::Mutex<ThreadModeRawMutex, Twim<'static, TWISPI0>>) {
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