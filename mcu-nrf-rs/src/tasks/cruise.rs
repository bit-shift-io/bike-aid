use crate::utils::signals;
use embassy_executor::Spawner;
use defmt::*;
use embassy_futures::select::{select, Either};

const TASK_ID: &str = "CRUISE";
const NO_THROTTLE_THRESHOLD: u16 = 1100;
const FULL_THROTTLE_THRESHOLD: u16 = 2700;
const MAX_COUNT: u8 = 6; // this equals X x 100ms of throttle updates

#[embassy_executor::task]
pub async fn task(
    spawner: Spawner,
) {
    info!("{}: start", TASK_ID);

    // spawn sub tasks
    spawner.must_spawn(cruise_reset());

    // brake on/off
    let mut sub = signals::BRAKE_ON.subscriber().unwrap();
    let mut state = false;

    loop { 
        if let Some(b) = sub.try_next_message_pure() {state = b}
        match state {
            false => {
                let sub_future = sub.next_message_pure();
                let task_future = run();
                match select(sub_future, task_future).await {
                    Either::First(val) => { state = val; }
                    Either::Second(_) => {} // other task will never end
                }
            },
            true => { state = sub.next_message_pure().await; }
        }
    }

}


#[embassy_executor::task]
async fn cruise_reset() {
    let mut sub_brake = signals::BRAKE_ON.subscriber().unwrap();

    loop {
        sub_brake.next_message_pure().await; // reset if brake on or off
        *signals::CRUISE_LEVEL.lock().await = 0;
        assign_voltage(0).await;
    }
}


async fn run() {
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
                // increment cruise level
                let mut cruise_level = signals::CRUISE_LEVEL.lock().await;
                let mut current_level = *cruise_level;
                current_level = (current_level + 1) % 5; // wrap around
                if current_level == 0 { current_level = 5; } // 0 -> 5 = range 1-5 instead of 0-4
                *cruise_level = current_level; // assign
                
                assign_voltage(current_level).await;

                pub_piezo.publish_immediate(signals::PiezoModeType::BeepShort);
                //info!("{}: Detected throttle tap + increment cruise 0, 1-5", TASK_ID);
            }
        }
    }
}


async fn assign_voltage(level: u8) {
    // assign voltage
    let cruise_voltages = *signals::CRUISE_VOLTAGES.lock().await;
    if level == 0 { *signals::CRUISE_VOLTAGE.lock().await = 0; }
    else { *signals::CRUISE_VOLTAGE.lock().await = cruise_voltages[(level -1) as usize]; } 
}