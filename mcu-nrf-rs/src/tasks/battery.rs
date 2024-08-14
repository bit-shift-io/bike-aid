use crate::utils::signals;
use defmt::*;

const TASK_ID: &str = "BATTERY";
const BATTERY_CAPACITY: u16 = 2400; // mah TODO: check capacity

#[embassy_executor::task]
pub async fn task() {
    info!("{}: start", TASK_ID);

    let pub_current = signals::BATTERY_CURRENT.publisher().unwrap();
    let pub_voltage = signals::BATTERY_VOLTAGE.publisher().unwrap();
    let pub_power = signals::BATTERY_POWER.publisher().unwrap();

    let mut sub_data = signals::BATTERY_IN.subscriber().unwrap();

    let voltage_data: [f64; 60] = [0.0; 60];
    let current_data: [f64; 60] = [0.0; 60];
    let mut power_sum: f64 = 0.0;
    let mut time_count = 0;

    loop {
        let input = sub_data.next_message_pure().await; // millivolts, updated 1 second
        let input_voltage = input[0];
        let input_current = input[1];
        
        let power = input_voltage * input_current; // milliwatts P=IV


        // every minute, transfer seconds reading to minutes
        time_count += 1;
        if time_count >= 60 {
            for i in 0..60 {
                let power = voltage_data[i] * current_data[i];
                power_sum += power;
            }

            let power_per_minute = power_sum / 60.0; // watt per minute
            let power_per_hour = power_per_minute * 60.0; // watt per hour

            time_count = 0;
        }


        pub_power.publish_immediate(power);
        pub_current.publish_immediate(input_current);
        pub_voltage.publish_immediate(input_voltage);
        info!("{}: current={} voltage={} power={}", TASK_ID, input_current, input_voltage, power);

        // TODO: calculate power every few seconds
        // store 1 minutes worth of data, and calulate smooth averge per minute
        // give an estimate of battery life, percentage and duration



    }
}