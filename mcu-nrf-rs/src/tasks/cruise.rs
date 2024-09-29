use crate::utils::signals;
use embassy_executor::Spawner;
use defmt::*;

const TASK_ID: &str = "CRUISE";
const NO_THROTTLE_THRESHOLD: u16 = 1100;
const FULL_THROTTLE_THRESHOLD: u16 = 2700;
const MAX_COUNT: u8 = 10; // this equals 1 seoncd of throttle updates

#[embassy_executor::task]
pub async fn task(
    spawner: Spawner,
) {
    info!("{}: start", TASK_ID);

    // spawn sub tasks
    spawner.must_spawn(cruise_reset());
    let mut sub_throttle = signals::THROTTLE_IN.subscriber().unwrap();
    let pub_piezo = signals::PIEZO_MODE.publisher().unwrap();

    loop {
        let mut throttle_voltage = sub_throttle.next_message_pure().await; // millivolts

        // go above the threshold
        if throttle_voltage > NO_THROTTLE_THRESHOLD {
            let mut count = 0;

            // Wait for the throttle to exceed the high threshold
            while throttle_voltage < FULL_THROTTLE_THRESHOLD && count < MAX_COUNT {
                throttle_voltage = sub_throttle.next_message_pure().await; // millivolts
                count += 1;
            }

            // Wait for the throttle to drop back below the low threshold
            while throttle_voltage > NO_THROTTLE_THRESHOLD && count < MAX_COUNT {
                throttle_voltage = sub_throttle.next_message_pure().await; // millivolts
                count +=1;
            }


            if count < MAX_COUNT {
                //info!("{}: Detected throttle tap + increment cruise 0, 1-5", TASK_ID);
                pub_piezo.publish_immediate(signals::PiezoModeType::Beep);
                // increment cruise - wrap around if larger than 5 cruise levels
                let mut cruise_level_lock = signals::CRUISE_LEVEL.lock().await;
                let mut current_level = *cruise_level_lock;
                current_level = (current_level + 1) % 5;
                if current_level == 0 { // treat 0 as 5 so we get range 1-5 instead of 0-4
                    current_level = 5;
                }
                *cruise_level_lock = current_level; // assign to mutex
            }
        }
    }
}


#[embassy_executor::task]
async fn cruise_reset() {
    let mut sub_brake = signals::BRAKE_ON.subscriber().unwrap();

    loop {
        if sub_brake.next_message_pure().await { // reset if brake on
            *signals::CRUISE_LEVEL.lock().await = 0;
        }
    }
}