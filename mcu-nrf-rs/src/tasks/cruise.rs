use crate::utils::{settings, signals};
use defmt::info;
use embassy_futures::select::select;
use embassy_time::Instant;

const TASK_ID: &str = "CRUISE";
const NO_THROTTLE_THRESHOLD: u16 = 1200;
const FULL_THROTTLE_THRESHOLD: u16 = 2600;
const BRAKE_TAP_TIME: u64 = 300; // ms
//const THROTTLE_TAP_TIME: u64 = 800; // ms
const THROTTLE_MAX_COUNT: u8 = 8; // this equals X x 100ms of throttle updates

#[embassy_executor::task]
pub async fn task() {
    info!("{}", TASK_ID);

    // throttle and brake dont work when power if off
    // but we may need to reset it when power if turned off?
    select(throttle_tap(), brake_tap()).await;
}


async fn brake_tap() {
    let mut rec_brake_on = signals::BRAKE_ON.receiver().unwrap();
    let send_piezo = signals::PIEZO_MODE.sender();

     // wait for brake off - not sure if this is needed?
     //rec_brake_on.changed_and(|b| *b == false).await;

    loop {
        // brake should be off here from boot & previous loop
        // wait for brake on
        rec_brake_on.changed_and(|b| *b == true).await;

        // start timing
        let time = Instant::now();

        // wait for brake off
        rec_brake_on.changed_and(|b| *b == false).await;

        if delta(time) < BRAKE_TAP_TIME {
            //info!("brake tap");
            decrement_cruise().await;
            send_piezo.send(signals::PiezoModeType::BeepShort);
        } else {
            // regaulr brake, so reset cruise
            set_cruise(0).await;
        }
    }
}


async fn throttle_tap() {
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
        while throttle_voltage < FULL_THROTTLE_THRESHOLD && count < THROTTLE_MAX_COUNT {
            throttle_voltage = rec_throttle.changed().await;
            count += 1;
        }

        // Now we are at full throttle, wait for it to drop below the NO_THROTTLE_THRESHOLD - throttle off
        while throttle_voltage > NO_THROTTLE_THRESHOLD && count < THROTTLE_MAX_COUNT {
            throttle_voltage = rec_throttle.changed().await;
            count += 1;
        }

        // Check if the tap was detected within the time limit
        if count < THROTTLE_MAX_COUNT {
            //info!("tap detected");
            increment_cruise().await;
            send_piezo.send(signals::PiezoModeType::BeepShort);
        }
    }
    /*
    loop {
        // should already be at no throttle theshold here! from previous boot & loop
        // Wait for throttle to go below the NO_THROTTLE_THRESHOLD - throttle off
        rec_throttle.changed_and(|v| *v < NO_THROTTLE_THRESHOLD).await;

        // Wait for the throttle to exceed the NO_THROTTLE_THRESHOLD - throttle start/low
        rec_throttle.changed_and(|v| *v > NO_THROTTLE_THRESHOLD).await;

        // start timing
        let time = Instant::now();

        // Wait for the throttle to exceed the FULL_THROTTLE_THRESHOLD - throttle high
        rec_throttle.changed_and(|v| *v > FULL_THROTTLE_THRESHOLD || delta(time) > THROTTLE_TAP_TIME).await;
        if delta(time) > THROTTLE_TAP_TIME { continue };

        // Now we are at full throttle, wait for it to drop below the NO_THROTTLE_THRESHOLD - throttle off
        rec_throttle.changed_and(|v| *v < NO_THROTTLE_THRESHOLD || delta(time) > THROTTLE_TAP_TIME).await;

        if delta(time) < THROTTLE_TAP_TIME {
            info!("throttle tap");
            increment_cruise().await;
            send_piezo.send(signals::PiezoModeType::BeepShort);
        }
    }
     */
}


pub fn delta(t: Instant) -> u64 {
    Instant::now().duration_since(t).as_millis()
}


async fn increment_cruise() {
    let mut level = signals::CRUISE_LEVEL.try_get().unwrap();

    // wrap around 0-4, move 0 -> 5 = range 1-5 instead of 0-4
    level = (level + 1) % 5; 
    if level == 0 { level = 5; }

    set_cruise(level).await;
}


async fn decrement_cruise() {
    let mut level = signals::CRUISE_LEVEL.try_get().unwrap();

    // wrap around 0-4, move 5 -> 0 = range 1-5 instead of 0-4
    level = (level - 1) % 5;
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