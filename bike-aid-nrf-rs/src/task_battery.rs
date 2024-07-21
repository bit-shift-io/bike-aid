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
    }
}