use crate::utils::{functions, signals};
use defmt::*;

const TASK_ID: &str = "BATTERY";
const BATTERY_CAPACITY: u16 = 2400; // mAh
const BATTERY_MAX_VOLTAGE: u16 = 54_600; // mV
const BATTERY_MIN_VOLTAGE: u16 = 39_000; // mv
const BATTERY_RANGE: u16 = BATTERY_MAX_VOLTAGE - BATTERY_MIN_VOLTAGE; // mv


#[embassy_executor::task]
pub async fn task() {
    info!("{}", TASK_ID);

    let pub_current = signals::BATTERY_CURRENT.publisher().unwrap();
    let pub_voltage = signals::BATTERY_VOLTAGE.publisher().unwrap();
    let pub_power = signals::BATTERY_POWER.publisher().unwrap();
    let pub_percent = signals::BATTERY_LEVEL.publisher().unwrap();

    let mut sub_data = signals::BATTERY_IN.subscriber().unwrap();

    let voltage_data: [f64; 60] = [0.0; 60];
    let current_data: [f64; 60] = [0.0; 60];
    let mut power_sum: f64 = 0.0;
    let mut time_count = 0;

    loop {
        let input = sub_data.next_message_pure().await; // millivolts, updated 1 second
        let input_current = input[1]; // mA
        let input_voltage = input[0]; // mV
        
        let percentage = calculate_percentage(input_voltage);

        // overflow, convert u32 then convert to larger units from ma
        //let power = (input_voltage as u32 * input_current as u32) as u16; // milliwatts P=IV - watts

        // TODO: calculate power every few seconds
        // store 1 minutes worth of data, and calulate smooth averge per minute
        // give an estimate of battery life, percentage and duration
        // // every minute, transfer seconds reading to minutes
        // time_count += 1;
        // if time_count >= 60 {
        //     for i in 0..60 {
        //         let power = voltage_data[i] * current_data[i];
        //         power_sum += power;
        //     }

        //     let power_per_minute = power_sum / 60.0; // watt per minute
        //     let power_per_hour = power_per_minute * 60.0; // watt per hour

        //     time_count = 0;
        // }

        //pub_power.publish_immediate(power);
        pub_current.publish_immediate(input_current);
        pub_voltage.publish_immediate(input_voltage);
        pub_percent.publish_immediate(percentage);
        //info!("{}: current:{} voltage:{} power:{} percent:{}", TASK_ID, input_current, input_voltage, power, percentage);
    }
}


fn calculate_percentage(voltage: u16) -> u8 {
        // battery cant be less than min, unless its not plugged in
        let voltage = functions::max(voltage, BATTERY_MIN_VOLTAGE);
        // Calculate the percentage using larger integer type to avoid overflow
        let voltage_adjusted = (voltage - BATTERY_MIN_VOLTAGE) as u32;
        (voltage_adjusted * 100 / BATTERY_RANGE as u32) as u8 // percent
}