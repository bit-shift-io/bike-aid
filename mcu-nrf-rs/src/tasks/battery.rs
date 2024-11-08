use crate::utils::signals;
use defmt::info;

const TASK_ID: &str = "BATTERY";
//const BATTERY_CAPACITY: u16 = 2400; // mAh
const BATTERY_MAX_VOLTAGE: u16 = 54_600; // mV
const BATTERY_MIN_VOLTAGE: u16 = 39_000; // mv
const BATTERY_RANGE: u16 = BATTERY_MAX_VOLTAGE - BATTERY_MIN_VOLTAGE; // mv


#[embassy_executor::task]
pub async fn task() {
    info!("{}", TASK_ID);

    let mut rec_data = signals::BATTERY_IN.receiver().unwrap();

    let mut last_power: u16 = 0;
    let mut last_percent: u8 = 0;

    loop {
        let input = rec_data.changed().await; // millivolts, updated 1 second
        let input_current = input[1]; // mA
        let input_voltage = input[0]; // mV
        
        let percent = calculate_percent(input_voltage).await;
        let power = calculate_power(input_voltage, input_current).await;

        if last_percent != percent {
            last_percent = percent;
            signals::send_ble(signals::BleHandles::BatteryLevel, percent.to_le_bytes().as_slice());
        }

        if last_power != power {
            last_power = power;
            signals::send_ble(signals::BleHandles::BatteryPower, power.to_le_bytes().as_slice());
        }

        //info!("{}: current:{} voltage:{} power:{} percent:{}", TASK_ID, input_current, input_voltage, power, percent);
    }
}


async fn calculate_percent(voltage: u16) -> u8 {
        // battery cant be less than min, unless its not plugged in
        let voltage = voltage.max(BATTERY_MIN_VOLTAGE); //functions::max(voltage, BATTERY_MIN_VOLTAGE);
        // Calculate the percentage using larger integer type to avoid overflow
        let voltage_adjusted = (voltage - BATTERY_MIN_VOLTAGE) as u32;
        (voltage_adjusted * 100 / BATTERY_RANGE as u32) as u8 // percent
}


async fn calculate_power(voltage: u16, current: u16) -> u16 {
    ((voltage as u32 * current as u32) / 1_000_000) as u16 // milliwatts P=IV - watts
}