use crate::utils::{i2c, signals};
use embassy_embedded_hal::shared_bus::asynch::i2c::I2cDevice;
use embassy_nrf::{peripherals::TWISPI0, twim::Twim};
use defmt::info;
use embassy_futures::select::{select, Either};
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::mutex;
use embedded_ads111x::{ADS111x, ADS111xConfig, DataRate, InputMultiplexer, ProgramableGainAmplifier};
use embassy_time::Timer;

const TASK_ID: &str = "BATTERY ADC";
const INTERVAL: u64 = 1; // seconds
const IDLE_INTERVAL: u64 = 3600; // 1 hr
const ADDRESS: u8 = 0x4A;

// consts for voltage divider
const VOLTAGE_CALIBATION : u16 = 10; // calibration level = multimeter - measured
const R_CALIBRATION : f32 = 0.050; // adjust resistor divider calibration
const R1 : f32 = 1_000_000.0; // 0_995_700.0
const R2 : f32 = 51_000.0; // 51_270.0
const VOLTAGE_MULTIPLIER : f32 = ((R1 + R2) / R2) - R_CALIBRATION; // ((R1 + R2) / R2)

// consts for ACS758LCB-100B
const VCC : u16 = 3300; // 3.3v = 3,300mV
const QUIESCENT_VOLTAGE : u16 = VCC / 2; // 0.5 (half) for ACS758LCB-100B
const SENSITIVITY: u16 = 100; // Sensitivity in mV/A for ACS758LCB-100B
const NON_ZERO: u16 = 7; // 7mV value to make voltage zero when there is no current
const MIN_CURRENT_LIMIT: u16 = 700; // mA


#[embassy_executor::task]
pub async fn task(
    i2c_bus: &'static mutex::Mutex<ThreadModeRawMutex, Twim<'static, TWISPI0>>
) {
    info!("{}", TASK_ID);

    // power on/off
    let mut rec_power_on = signals::POWER_ON.receiver().unwrap();
    let mut state_power_on = rec_power_on.try_get().unwrap();

    loop { 
        match select(rec_power_on.changed(), battery_adc(i2c_bus, state_power_on)).await {
            Either::First(b) => { state_power_on = b; },
            Either::Second(_) => {}
        }
    }
}


async fn battery_adc(
    i2c_bus: &'static mutex::Mutex<ThreadModeRawMutex, Twim<'static, TWISPI0>>,
    power_on: bool) {

    // check if device available
    if !i2c::device_available(i2c_bus, ADDRESS).await {
        info!("{}: end", TASK_ID);
        Timer::after_secs(3600).await; // retry after 1 hr
        return;
    }

    // set update interval
    let interval = if power_on { INTERVAL } else { IDLE_INTERVAL };

    // init device
    let i2c = I2cDevice::new(i2c_bus);
    let config = ADS111xConfig::default()
        .mux(InputMultiplexer::AIN0GND)
        .dr(DataRate::SPS860)
        .pga(ProgramableGainAmplifier::V6_144); // 6.144v

    let mut adc = match ADS111x::new(i2c, ADDRESS, config) { // 0x4A
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
    
    let send_data = signals::BATTERY_IN.sender();

    loop {
        Timer::after_secs(interval).await;

        // read 2 values takes 6ms
        let value_a0 = adc.read_single_voltage(Some(InputMultiplexer::AIN0GND)).await; // current
        let value_a1 = adc.read_single_voltage(Some(InputMultiplexer::AIN1GND)).await; // voltage

        if value_a0.is_err() || value_a1.is_err() {
            info!("{}: device error", TASK_ID);
            continue
        };

        let voltage = calculate_voltage(value_a1.unwrap());
        let current = calculate_current(value_a0.unwrap());
        send_data.send([voltage, current]);

        //info!("{}: voltage: {} current: {}", TASK_ID, voltage, current);
    }
}


fn calculate_voltage(input: f32) -> u16 {
    // convert to voltage
    // ADC - 4.096v * 1000 (to mv) / 32768 (15 bit, 1 bit +-)
    //let mut input_voltage_a1: u16 = (input * 4096.0 / 32768.0) as u16; // converted to mv
    let mut input_voltage = (input * 1000f32) as u16; // mv

    // calibration
    input_voltage += VOLTAGE_CALIBATION; 

    //info!("{}: a0: {} -> {}, a1: {} -> {}", TASK_ID, value_a0, input_voltage_a0, value_a1, input_voltage_a1);
    //info!("{}: multiplier: {}", TASK_ID, VOLTAGE_MULTIPLIER);
    
    // voltage before the resitor divider
    (f32::from(input_voltage) * VOLTAGE_MULTIPLIER) as u16 // mv
}


fn calculate_current(input: f32) -> u16 {
    // convert to voltage
    // ADC - 4.096v * 1000 (to mv) / 32768 (15 bit, 1 bit +-)
    //let input_voltage_a0: u16 = (input * 4096.0 / 32768.0) as u16; // converted to mv
    let input_voltage = (input * 1000f32) as u16; // mv

    let differential_voltage = input_voltage - QUIESCENT_VOLTAGE + NON_ZERO;
    let mut current = ((1000 * differential_voltage as u32) / SENSITIVITY as u32) as u16; // mA - u32 prevent overflow
    if current < MIN_CURRENT_LIMIT {
         current = 0;
    }
    
    //info!("{} -> {} -> {} -> {}", input, input_voltage_a0, differential_voltage, current);
    current
}