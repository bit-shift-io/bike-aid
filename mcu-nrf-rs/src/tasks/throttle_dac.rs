use core::future;

use crate::utils::{i2c, signals};
use embassy_embedded_hal::shared_bus::asynch::i2c::I2cDevice;
use embassy_embedded_hal::shared_bus::I2cDeviceError;
use embassy_nrf::{peripherals::TWISPI0, twim::Twim};
use defmt::info;
use embassy_futures::select::{select3, Either3};
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::mutex;
use embassy_time::Timer;
use mcp4725_async::MCP4725;

const TASK_ID : &str = "THROTTLE DAC";
const ADDRESS: u8 = 0x60;
// TODO: mv supply for calibration
// set just under voltage across the g and v pins of the throttle module
// Controller supply voltage - 4.36v = 4360mv
// onboard power supply - 4.98v = 4980mv
const SUPPLY_VOLTAGE: u16 = 5000; // mv
const DAC_MAX: u16 = 4095; // 15 bits

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
    if park_brake_on || !power_on { stop(i2c_bus).await }
    if power_on && !park_brake_on { throttle_dac(i2c_bus).await }
    future::pending().await // wait/yield forever doing nothing
}


async fn throttle_dac(i2c_bus: &'static mutex::Mutex<ThreadModeRawMutex, Twim<'static, TWISPI0>>) {
    // check if device available
    if !i2c::device_available(i2c_bus, ADDRESS).await {
        info!("{}: end", TASK_ID);
        return;
    }

    // init device
    let mut dac = match get_dac(i2c_bus).await {
        Ok(x) => { x },
        Err(_e) => {
            info!("{}: device error", TASK_ID);
            return;
        }
    };

    let mut last_value = 0;
    let mut rec_throttle = signals::THROTTLE_OUT.receiver().unwrap();

    loop {
        let value = rec_throttle.changed().await; // desired mv
        
        // dont reapply same value
        if value == last_value { continue; };
        last_value = value;

        // using larger integer type to avoid overflow
        // note: changed from f32 to u32
        let dac_value = (value as u32 * DAC_MAX as u32 / SUPPLY_VOLTAGE as u32) as u16;
        let dac_value = dac_value.min(DAC_MAX);

        match dac.set_voltage(dac_value, false).await {
            Ok(_) => {},
            Err(_e) => {
                info!("{}: device error", TASK_ID);
                return;
            }
        }

        //info!("{} : {}", TASK_ID, dac_value); // dac value, not in mv
    }
}


async fn stop(i2c_bus: &'static mutex::Mutex<ThreadModeRawMutex, Twim<'static, TWISPI0>>) {
    let _ = get_dac(i2c_bus).await; // will reset it also
}


async fn get_dac(i2c_bus: &'static mutex::Mutex<ThreadModeRawMutex, Twim<'static, TWISPI0>>) -> Result<MCP4725<I2cDevice<'_, ThreadModeRawMutex, Twim<'_, TWISPI0>>>, I2cDeviceError<embassy_nrf::twim::Error>>{
    let i2c = I2cDevice::new(i2c_bus);

    // Address corresponds to A2,A1=0, and A0 tied to Vss
    //let mut dac = MCP4725::new(i2c, 0b1100000);
    let mut dac = MCP4725::new(i2c, ADDRESS);
    
    // Set DAC to 0x000 = Zero volts, write to eeprom
    match dac.set_voltage(0x000, true).await {
        Ok(_) => { return Ok(dac); },
        Err(e) => {
            info!("{}: device error", TASK_ID);
            return Err(e);
        }, // unable to communicate with device
    }
}


#[allow(unused)]
async fn calibrate(i2c_bus: &'static mutex::Mutex<ThreadModeRawMutex, Twim<'static, TWISPI0>>) {
    let mut dac = match get_dac(i2c_bus).await {
        Ok(x) => { x },
        Err(_e) => {
            info!("{}: device error", TASK_ID);
            return;
        }
    };

    let v0 = 0u16;
    let v1 = (1000.0 * 4095.0 / f32::from(SUPPLY_VOLTAGE)) as u16;
    let v2 = (2000.0 * 4095.0 / f32::from(SUPPLY_VOLTAGE)) as u16;
    let v3 = (3000.0 * 4095.0 / f32::from(SUPPLY_VOLTAGE)) as u16;
    let v4 = (4000.0 * 4095.0 / f32::from(SUPPLY_VOLTAGE)) as u16;
    let vcc = 4095u16;

    loop {
        // testing calibration
        let _ = dac.set_voltage(v0, false).await;
        info!("{} : 0V", TASK_ID);
        Timer::after_secs(5).await;
        let _ = dac.set_voltage(v1, false).await;
        info!("{} : 1V", TASK_ID);
        Timer::after_secs(5).await;
        let _ = dac.set_voltage(v2, false).await;
        info!("{} : 2V", TASK_ID);
        Timer::after_secs(5).await;
        let _ = dac.set_voltage(v3, false).await;
        info!("{} : 3V", TASK_ID);
        Timer::after_secs(5).await;
        let _ = dac.set_voltage(v4, false).await;
        info!("{} : 4V", TASK_ID);
        Timer::after_secs(5).await;
        let _ = dac.set_voltage(vcc, false).await;
        info!("{} : 5V", TASK_ID);
        Timer::after_secs(5).await;
    }
}