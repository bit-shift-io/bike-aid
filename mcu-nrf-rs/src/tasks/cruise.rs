use crate::utils::signals;
use embassy_time::Timer;
use defmt::*;
use embassy_futures::select::{select, Either};

const TASK_ID: &str = "CRUISE";
const INTERVAL: u64 = 500; // 0.5 sec
const MIN_VOLTAGE: u16 = 1200; // do we want to move this to settings?
const RANGE: u16 = 200;

#[embassy_executor::task]
pub async fn task() {
    info!("{}: start", TASK_ID);

    let mut sub_cruise_enabled = signals::CRUISE_ENABLED.subscriber().unwrap();
    let mut cruise_state = false;

    loop { 
        if let Some(b) = sub_cruise_enabled.try_next_message_pure() {cruise_state = b}
        match cruise_state {
            true => {
                let cruise_future = sub_cruise_enabled.next_message_pure();
                let task_future = run();
                match select(cruise_future, task_future).await {
                    Either::First(val) => { cruise_state = val; }
                    Either::Second(_) => { Timer::after_secs(60).await; } // retry
                }
            },
            false => { 
                cruise_state = sub_cruise_enabled.next_message_pure().await; 
            }
        }
    }
}


async fn run() {
    let mut sub_throttle = signals::THROTTLE_IN.subscriber().unwrap();
    let pub_cruise_level = signals::CRUISE_VOLTAGE.publisher().unwrap();
    let pub_cruise_enabled = signals::CRUISE_ENABLED.publisher().unwrap();
    let pub_piezo = signals::PIEZO_MODE.publisher().unwrap();
    let mut data: [u16; 6] = [0; 6];
    let mut index = 0;

    loop {
        Timer::after_millis(INTERVAL).await;
        let throttle_voltage = sub_throttle.next_message_pure().await; // millivolts

        if throttle_voltage >= MIN_VOLTAGE {
            data[index] = throttle_voltage;
            index = (index + 1) % data.len(); // increment index, wrap around if larger than size

            let min = data.iter().min().unwrap();
            let max = data.iter().max().unwrap();
            let diff = max - min;

            //info!("{}: min: {}, max: {}, diff: {}", TASK_ID, min, max, diff);
            if diff <= RANGE {
                pub_cruise_level.publish_immediate(*max); // max value is the cruise voltage
                pub_cruise_enabled.publish_immediate(true);
                pub_piezo.publish_immediate(signals::PiezoModeType::RydeOfTheWalkyries);
            }
        }
    }
}
