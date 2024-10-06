use crate::utils::signals;
use crate::utils::functions;
use embassy_embedded_hal::shared_bus::blocking::i2c::I2cDevice;
use embassy_nrf::{peripherals::TWISPI0, twim::Twim};
use defmt::*;
use embassy_futures::select::{select, Either};
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::blocking_mutex::Mutex;
use embassy_time::Timer;
use core::cell::RefCell;
use mcp4725::MCP4725;

const TASK_ID : &str = "THROTTLE DAC";
const ADDRESS: u8 = 0x60;
// TODO: mv supply for calibration
// set just under voltage across the g and v pins of the throttle module
// Controller supply voltage - 4.36v = 4360mv
// onboard power supply - 4.98v = 4980mv
const SUPPLY_VOLTAGE: u16 = 5000; // mv

#[embassy_executor::task]
pub async fn task(
    i2c_bus: &'static Mutex<NoopRawMutex, RefCell<Twim<'static, TWISPI0>>>
) {
    info!("{}: start", TASK_ID);
  
    let mut sub = signals::SWITCH_POWER.subscriber().unwrap();
    let mut state = false;

    loop { 
        if let Some(b) = sub.try_next_message_pure() {state = b}
        match state {
            true => {
                let sub_future = sub.next_message_pure();
                let task_future = park_brake(i2c_bus);
                match select(sub_future, task_future).await {
                    Either::First(val) => { state = val; }
                    Either::Second(_) => { Timer::after_secs(60).await; } // retry
                }
            },
            false => { 
                stop(i2c_bus).await; // set power to device off
                state = sub.next_message_pure().await; 
            }
        }
    }
}


async fn park_brake(i2c_bus: &'static Mutex<NoopRawMutex, RefCell<Twim<'static, TWISPI0>>>) {
    // park brake on/off
    let mut sub = signals::PARK_BRAKE_ON.subscriber().unwrap();
    let mut state = true; // default to on

    loop { 
        if let Some(b) = sub.try_next_message_pure() {state = b}
        match state {
            false => {
                let sub_future = sub.next_message_pure();
                let task_future = run(i2c_bus);
                match select(sub_future, task_future).await {
                    Either::First(val) => { state = val; }
                    Either::Second(_) => { Timer::after_secs(60).await; } // retry
                }
            },
            true => { 
                stop(i2c_bus).await; // set power to device off
                state = sub.next_message_pure().await; 
            }
        }
    }
}


async fn calibrate(
    i2c_bus: &'static Mutex<NoopRawMutex, 
    RefCell<Twim<'static, TWISPI0>>>
) {
    let i2c = I2cDevice::new(i2c_bus);
    let mut dac = MCP4725::new(i2c, ADDRESS);
    let result = dac.set_dac_and_eeprom(mcp4725::PowerDown::Normal, 0); // set 0 volts output
    match result {
        Ok(()) => {},
        Err(_e) => {
            info!("{} : device error", TASK_ID);
            return
        }, // unable to communicate with device
    }

    loop {
        // testing calibration
        let _ = dac.set_dac(mcp4725::PowerDown::Normal, (1000.0 * 4095.0 / f32::from(SUPPLY_VOLTAGE)) as u16);    //Set voltage to 1V
        info!("{} : 1V", TASK_ID);
        Timer::after_secs(5).await;
        let _ = dac.set_dac(mcp4725::PowerDown::Normal, (2000.0 * 4095.0 / f32::from(SUPPLY_VOLTAGE)) as u16);    //Set voltage to 2V
        info!("{} : 2V", TASK_ID);
        Timer::after_secs(5).await;
        let _ = dac.set_dac(mcp4725::PowerDown::Normal, (3000.0 * 4095.0 / f32::from(SUPPLY_VOLTAGE)) as u16);    //Set voltage to 3V
        info!("{} : 3V", TASK_ID);
        Timer::after_secs(5).await;
        let _ = dac.set_dac(mcp4725::PowerDown::Normal, (4000.0 * 4095.0 / f32::from(SUPPLY_VOLTAGE)) as u16);    //Set voltage to 4V
        info!("{} : 4V", TASK_ID);
        Timer::after_secs(5).await;
        let _ = dac.set_dac(mcp4725::PowerDown::Normal, (4095.0) as u16);    //Set voltage to 5V or (Vcc)
        info!("{} : 5V", TASK_ID);
        Timer::after_secs(5).await;
    }
}


async fn run(
    i2c_bus: &'static Mutex<NoopRawMutex, 
    RefCell<Twim<'static, TWISPI0>>>
) {
    let i2c = I2cDevice::new(i2c_bus);
    let mut sub_throttle = signals::THROTTLE_OUT.subscriber().unwrap();
    let mut dac = MCP4725::new(i2c, ADDRESS);
    let result = dac.set_dac_and_eeprom(mcp4725::PowerDown::Normal, 0); // set 0 volts output
    match result {
        Ok(()) => {},
        Err(_e) => {
            info!("{} : device error", TASK_ID);
            return
        }, // unable to communicate with device
    }

    loop {
        let value = sub_throttle.next_message_pure().await; // desired mv
        let dac_value = (f32::from(value) * 4095.0 / SUPPLY_VOLTAGE as f32) as u16;
        let dac_value = functions::min(4095, dac_value); // 4095 is supply voltage, cant go above this
        let _ = dac.set_dac(mcp4725::PowerDown::Normal, dac_value as u16);
        //info!("{} : {}", TASK_ID, dac_value); // dac value, not in mv
    }
}


async fn stop(
    i2c_bus: &'static Mutex<NoopRawMutex, 
    RefCell<Twim<'static, TWISPI0>>>
) {
    let i2c = I2cDevice::new(i2c_bus);
    let mut dac = MCP4725::new(i2c, ADDRESS);
    let result = dac.set_dac_and_eeprom(mcp4725::PowerDown::Normal, 0); // set 0 volts output
    match result {
        Ok(()) => {},
        Err(_e) => {
            info!("{} : device error", TASK_ID);
            return
        }, // unable to communicate with device
    }
}