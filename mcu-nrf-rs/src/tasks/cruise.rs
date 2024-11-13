use crate::utils::{settings, signals};
use defmt::info;
use embassy_futures::select::select;

const TASK_ID: &str = "CRUISE";
const NO_THROTTLE_THRESHOLD: u16 = 1200;
const FULL_THROTTLE_THRESHOLD: u16 = 2600;
const MAX_COUNT: u8 = 8; // this equals X x 100ms of throttle updates

#[embassy_executor::task]
pub async fn task() {
    info!("{}", TASK_ID);

    // brake on/off
    let mut rec_brake_on = signals::BRAKE_ON.receiver().unwrap();

    loop { 
        if !rec_brake_on.changed().await { // brake off
            select(rec_brake_on.changed(), tap_detection()).await;
            set_cruise(0).await;
        }
    }
}


async fn tap_detection() {
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
    let mut level = signals::CRUISE_LEVEL.try_get().unwrap();

    // wrap around 0-4, move 0 -> 5 = range 1-5 instead of 0-4
    level = (level + 1) % 5; 
    if level == 0 { level = 5; }

    set_cruise(level).await;
}


async fn set_cruise(level: u8) {
    let current_level = signals::CRUISE_LEVEL.try_get().unwrap();

    // only send if changed
    if current_level != level {
        // set voltage
        let cruise_voltages = *settings::CRUISE_VOLTAGES.lock().await;
        let voltage = if level == 0 { 0 } else { cruise_voltages[(level -1) as usize] }; 
        settings::CRUISE_VOLTAGE.dyn_sender().send(voltage);

        // set level
        signals::CRUISE_LEVEL.dyn_sender().send(level);
        signals::send_ble(signals::BleHandles::CruiseLevel, &[level]);
    }
}