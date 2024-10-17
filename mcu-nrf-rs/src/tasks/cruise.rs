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
    let mut rec_throttle = signals::THROTTLE_IN_WATCH.receiver().unwrap();
    let send_piezo = signals::PIEZO_MODE_WATCH.sender();

    loop {
        let mut throttle_voltage = rec_throttle.changed().await; // millivolts

        // Wait for throttle to go below the NO_THROTTLE_THRESHOLD
        while throttle_voltage >= NO_THROTTLE_THRESHOLD {
            throttle_voltage = rec_throttle.changed().await; // millivolts
        }

        // Now we are below the NO_THROTTLE_THRESHOLD, wait for it to exceed the threshold
        //info!("below no throttle threshold");

        // Wait for the throttle to exceed the NO_THROTTLE_THRESHOLD
        while throttle_voltage < NO_THROTTLE_THRESHOLD {
            throttle_voltage = rec_throttle.changed().await; // millivolts
        }

        // Count for timing, each update is 100ms
        let mut count = 0;

        //info!("above min throttle - {}", count);

        // Wait for the throttle to exceed the FULL_THROTTLE_THRESHOLD
        while throttle_voltage < FULL_THROTTLE_THRESHOLD && count < MAX_COUNT {
            throttle_voltage = rec_throttle.changed().await; // millivolts
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
            throttle_voltage = rec_throttle.changed().await; // millivolts
            count += 1;
        }

        //info!("throttle dropped below no throttle threshold, count: {}", count);

        // Check if the tap was detected within the time limit (0.6 seconds)
        if count < MAX_COUNT {
            //info!("tap detected");
            increment_cruise().await;
            send_piezo.send(signals::PiezoModeType::BeepShort);
        }
    }
}


async fn increment_cruise() {
    let mut rec_cruise_level = signals::CRUISE_LEVEL_WATCH.receiver().unwrap();
    let mut current_level = rec_cruise_level.try_get().unwrap();

    // wrap around 0-4, move 0 -> 5 = range 1-5 instead of 0-4
    current_level = (current_level + 1) % 5; 
    if current_level == 0 { current_level = 5; }

    let send_cruise_level = signals::CRUISE_LEVEL_WATCH.sender();
    send_cruise_level.send(current_level);
    signals::send_ble(2, signals::BleHandles::CruiseLevel, current_level.to_le_bytes().as_slice());

    assign_voltage(current_level).await;
}


async fn assign_voltage(level: u8) {
    let cruise_voltages = *signals::CRUISE_VOLTAGES_MUTEX.lock().await;
    if level == 0 { *signals::CRUISE_VOLTAGE_MUTEX.lock().await = 0; }
    else { *signals::CRUISE_VOLTAGE_MUTEX.lock().await = cruise_voltages[(level -1) as usize]; } 
}


async fn reset_cruise() {
    signals::CRUISE_LEVEL_WATCH.dyn_sender().send_if_modified(|value| {
        if *value != Some(0) {
            *value = Some(0);
            true
        } else { false } // no change
    });
    signals::send_ble(2, signals::BleHandles::CruiseLevel, &[0u8]);
    assign_voltage(0).await;
}