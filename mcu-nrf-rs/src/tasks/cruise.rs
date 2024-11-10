use crate::utils::{settings, signals};
use defmt::info;
use embassy_futures::select::{select, Either};

const TASK_ID: &str = "CRUISE";
const NO_THROTTLE_THRESHOLD: u16 = 1200;
const FULL_THROTTLE_THRESHOLD: u16 = 2600;
const MAX_COUNT: u8 = 8; // this equals X x 100ms of throttle updates

#[embassy_executor::task]
pub async fn task() {
    info!("{}", TASK_ID);

    // brake on/off
    let mut watch = signals::BRAKE_ON.receiver().unwrap();
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
    // tap detection
    let mut rec_throttle = signals::THROTTLE_IN.receiver().unwrap();
    let send_piezo = signals::PIEZO_MODE.sender();
    let mut throttle_voltage = rec_throttle.changed().await; // millivolts, initial value

    loop {
        // Wait for throttle to go below the NO_THROTTLE_THRESHOLD - throttle off
        while throttle_voltage > NO_THROTTLE_THRESHOLD {
            throttle_voltage = rec_throttle.changed().await;
        }

        // Wait for the throttle to exceed the NO_THROTTLE_THRESHOLD - throttle start/low
        while throttle_voltage < NO_THROTTLE_THRESHOLD {
            throttle_voltage = rec_throttle.changed().await;
        }

        // start timing, each update is 100ms
        let mut count = 0;

        // Wait for the throttle to exceed the FULL_THROTTLE_THRESHOLD - throttle high
        while throttle_voltage < FULL_THROTTLE_THRESHOLD && count < MAX_COUNT {
            throttle_voltage = rec_throttle.changed().await;
            count += 1;
        }

        // Now we are at full throttle, wait for it to drop below the NO_THROTTLE_THRESHOLD - throttle off
        while throttle_voltage > NO_THROTTLE_THRESHOLD && count < MAX_COUNT {
            throttle_voltage = rec_throttle.changed().await;
            count += 1;
        }

        // Check if the tap was detected within the time limit
        if count < MAX_COUNT {
            //info!("tap detected");
            increment_cruise().await;
            send_piezo.send(signals::PiezoModeType::BeepShort);
        }
    }
}


async fn increment_cruise() {
    let mut rec_cruise_level = signals::CRUISE_LEVEL.receiver().unwrap();
    let mut current_level = rec_cruise_level.try_get().unwrap();

    // wrap around 0-4, move 0 -> 5 = range 1-5 instead of 0-4
    current_level = (current_level + 1) % 5; 
    if current_level == 0 { current_level = 5; }

    let send_cruise_level = signals::CRUISE_LEVEL.sender();
    send_cruise_level.send(current_level);
    signals::send_ble(signals::BleHandles::CruiseLevel, current_level.to_le_bytes().as_slice());

    assign_voltage(current_level).await;
}


async fn assign_voltage(level: u8) {
    let cruise_voltages = *settings::CRUISE_VOLTAGES.lock().await;
    if level == 0 { *settings::CRUISE_VOLTAGE.lock().await = 0; }
    else { *settings::CRUISE_VOLTAGE.lock().await = cruise_voltages[(level -1) as usize]; } 
}


async fn reset_cruise() {
    let mut rec_cruise_level = signals::CRUISE_LEVEL.receiver().unwrap();
    let current_level = rec_cruise_level.try_get().unwrap();

    // only send if changed
    if current_level != 0 {
        signals::CRUISE_LEVEL.dyn_sender().send(0);
        signals::send_ble(signals::BleHandles::CruiseLevel, &[0u8]);
        assign_voltage(0).await;
    }
}