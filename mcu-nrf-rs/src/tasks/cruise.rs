use crate::utils::signals;
use defmt::*;
use embassy_futures::select::{select, Either};

const TASK_ID: &str = "CRUISE";
const NO_THROTTLE_THRESHOLD: u16 = 1100;
const FULL_THROTTLE_THRESHOLD: u16 = 2700;
const MAX_COUNT: u8 = 6; // this equals X x 100ms of throttle updates

#[embassy_executor::task]
pub async fn task() {
    info!("{}", TASK_ID);

    // brake on/off
    let mut watch = signals::BRAKE_ON_WATCH.receiver().unwrap();
    let mut state = false;

    loop { 
        if let Some(b) = watch.try_get() {state = b}
        
        match state {
            false => {
                let watch_future = watch.changed();
                let task_future = run();
                //let task2_future = cruise_reset();
                //let task_future = join(task1_future, task2_future);
                match select(watch_future, task_future).await {
                    Either::First(val) => { state = val; }
                    Either::Second(_) => {} // other task will never end
                }
            },
            true => { 
                reset_cruise().await;
                state = watch.changed().await;
            }
        }
    }

}


async fn run() {
    let mut sub_throttle = signals::THROTTLE_IN.subscriber().unwrap();
    let pub_piezo = signals::PIEZO_MODE.publisher().unwrap();

    loop {
        let mut throttle_voltage = sub_throttle.next_message_pure().await; // millivolts

        // Wait for throttle to go below the NO_THROTTLE_THRESHOLD
        while throttle_voltage >= NO_THROTTLE_THRESHOLD {
            throttle_voltage = sub_throttle.next_message_pure().await; // millivolts
        }

        // Now we are below the NO_THROTTLE_THRESHOLD, wait for it to exceed the threshold
        //info!("below no throttle threshold");

        // Wait for the throttle to exceed the NO_THROTTLE_THRESHOLD
        while throttle_voltage < NO_THROTTLE_THRESHOLD {
            throttle_voltage = sub_throttle.next_message_pure().await; // millivolts
        }

        // Count for timing, each update is 100ms
        let mut count = 0;

        //info!("above min throttle - {}", count);

        // Wait for the throttle to exceed the FULL_THROTTLE_THRESHOLD
        while throttle_voltage < FULL_THROTTLE_THRESHOLD && count < MAX_COUNT {
            throttle_voltage = sub_throttle.next_message_pure().await; // millivolts
            //info!("throttle {}", throttle_voltage);
            count += 1;
        }

        //info!("full throttle {}", count);

        if count >= MAX_COUNT {
            continue; // Restart the loop if we didn't detect a full throttle
        }

        // Now we are at full throttle, wait for it to drop below the NO_THROTTLE_THRESHOLD
        //count = 0; // Reset count for timing the drop
        while throttle_voltage > NO_THROTTLE_THRESHOLD && count < MAX_COUNT {
            throttle_voltage = sub_throttle.next_message_pure().await; // millivolts
            count += 1;
        }

        //info!("throttle dropped below no throttle threshold, count: {}", count);

        // Check if the tap was detected within the time limit (0.6 seconds)
        if count < MAX_COUNT {
            //info!("tap detected");
            increment_cruise().await;
            pub_piezo.publish_immediate(signals::PiezoModeType::BeepShort);
        }
    }
}


async fn increment_cruise() {
    // old
    let mut cruise_level = signals::CRUISE_LEVEL.lock().await;
    let mut current_level = *cruise_level;

    // wrap around 0-4, move 0 -> 5 = range 1-5 instead of 0-4
    current_level = (current_level + 1) % 5; 
    if current_level == 0 { current_level = 5; }

    *cruise_level = current_level; // assign old

    // new
    let sen_cruise_level = signals::CRUISE_LEVEL_WATCH.sender();
    sen_cruise_level.send(current_level);


    assign_voltage(current_level).await;
}


async fn assign_voltage(level: u8) {
    let cruise_voltages = *signals::CRUISE_VOLTAGES.lock().await;
    if level == 0 { *signals::CRUISE_VOLTAGE.lock().await = 0; }
    else { *signals::CRUISE_VOLTAGE.lock().await = cruise_voltages[(level -1) as usize]; } 
}


async fn reset_cruise() {
    *signals::CRUISE_LEVEL.lock().await = 0; // old method
    signals::CRUISE_LEVEL_WATCH.dyn_sender().send_if_modified(|value| {
        if *value != Some(0) {
            *value = Some(0); // Set the value to 0 only if it is not already 0
            true // Indicate that the value was modified
        } else {
            false // No modification made
        }
    });
    assign_voltage(0).await;
}