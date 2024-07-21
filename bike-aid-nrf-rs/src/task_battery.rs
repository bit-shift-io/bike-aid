use crate::signals;
use defmt::*;

const TASK_ID: &str = "BATTERY";

#[embassy_executor::task]
pub async fn battery () {
    info!("{}: start", TASK_ID);

    let pub_current = signals::BATTERY_CURRENT.publisher().unwrap();
    let pub_voltage = signals::BATTERY_VOLTAGE.publisher().unwrap();
    let pub_power = signals::BATTERY_POWER.publisher().unwrap();

    let mut sub_current = signals::BATTERY_CURRENT_IN.subscriber().unwrap();
    let mut sub_voltage = signals::BATTERY_VOLTAGE_IN.subscriber().unwrap();

    loop {
        let input_voltage = sub_voltage.next_message_pure().await; // millivolts
        let input_current = sub_current.next_message_pure().await; // millivolts
        let power = input_voltage * input_current; // milliwatts P=IV
        pub_power.publish_immediate(power);
        pub_current.publish_immediate(input_current);
        pub_voltage.publish_immediate(input_voltage);
        info!("{}: current={} voltage={} power={}", TASK_ID, input_current, input_voltage, power);

        // TODO: calculate power every few seconds
        // store 1 minutes worth of data, and calulate smooth averge per minute
        // give an estimate of battery life, percentage and duration


    }
}