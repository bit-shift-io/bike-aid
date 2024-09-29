use crate::utils::signals;
use embassy_time::Timer;
use defmt::*;

const TASK_ID: &str = "CRUISE";
const INTERVAL: u64 = 500; // 0.5 sec
const MIN_VOLTAGE: u16 = 1200; // do we want to move this to settings?
const RANGE: u16 = 200;

#[embassy_executor::task]
pub async fn task() {
    info!("{}: start", TASK_ID);
    let mut sub_throttle = signals::THROTTLE_IN.subscriber().unwrap();
    let mut data: [u16; 6] = [0; 6];
    let mut index = 0;

    loop {
        Timer::after_millis(INTERVAL).await;
        let throttle_voltage = sub_throttle.next_message_pure().await; // millivolts
        let pub_piezo = signals::PIEZO_MODE.publisher().unwrap();

        if throttle_voltage >= MIN_VOLTAGE {
            data[index] = throttle_voltage;
            index = (index + 1) % data.len(); // increment index, wrap around if larger than size

            let min = data.iter().min().unwrap();
            let max = data.iter().max().unwrap();
            let diff = max - min;

            //info!("{}: min: {}, max: {}, diff: {}", TASK_ID, min, max, diff);
            if diff <= RANGE {
                info!("{}: cruise enabled", TASK_ID);
                pub_piezo.publish_immediate(signals::PiezoModeType::RydeOfTheWalkyries);
            }
        }

    }
}
