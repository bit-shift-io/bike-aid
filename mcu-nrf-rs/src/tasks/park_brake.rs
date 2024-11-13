use core::future;
use crate::utils::signals;
use defmt::info;
use embassy_futures::select::{select4, Either4};

const TASK_ID: &str = "PARK BRAKE";
const NO_THROTTLE_THRESHOLD: u16 = 1100;
const MAX_COUNT: u16 = 30 * 10; // this equals 30 seonds of throttle updates

#[embassy_executor::task]
pub async fn task() {
    info!("{}", TASK_ID);

    let mut rec_power_on = signals::POWER_ON.receiver().unwrap();
    let mut state_power_on = rec_power_on.try_get().unwrap();

    let mut rec_park_brake_on = signals::PARK_BRAKE_ON.receiver().unwrap();
    let mut state_park_brake_on = rec_park_brake_on.try_get().unwrap();

    let mut rec_cruise_level = signals::CRUISE_LEVEL.receiver().unwrap();
    let mut state_cruise_level = rec_cruise_level.try_get().unwrap();

    loop {
        match select4(rec_power_on.changed(), rec_park_brake_on.changed(), rec_cruise_level.changed(), run(state_power_on, state_park_brake_on, state_cruise_level)).await {
            Either4::First(b) => { state_power_on = b; },
            Either4::Second(b) => { state_park_brake_on = b; },
            Either4::Third(b) => { state_cruise_level = b; }
            Either4::Fourth(_) => {}
        }
    }
}


pub async fn run(power_on: bool, park_brake_on: bool, cruise_level: u8) {
    // power off
    if !power_on { reset().await; }

    // power off || cruise on - wait for change
    if !(power_on && cruise_level == 0) { future::pending().await }

    // park_brake_on - false/off
    match park_brake_on {
        true => { wait_park_brake_off().await; },
        false => { wait_park_brake_on().await; }
    }
}


async fn wait_park_brake_on() {
    // detect when to turn park brake on
    let send_piezo = signals::PIEZO_MODE.sender();
    let watch_park_brake_on = signals::PARK_BRAKE_ON.sender();
    let mut rec_throttle = signals::THROTTLE_IN.receiver().unwrap();
    let mut count = 0;

    loop {
        let throttle_voltage = rec_throttle.changed().await; // millivolts

        // detect park brake on
        if throttle_voltage < NO_THROTTLE_THRESHOLD {
            count += 1;

            if count > MAX_COUNT {
                send_piezo.send(signals::PiezoModeType::BeepLong);
                watch_park_brake_on.send(true);
                signals::send_ble(signals::BleHandles::ParkBrakeOn, &[true as u8]);
                //info!("on: park brake on");
                return;
            }
        } else {
            count = 0;
        }
    }
}


async fn wait_park_brake_off() {
    // wait for brake to be on
    let mut watch_brake_on = signals::BRAKE_ON.receiver().unwrap();
    let _ = watch_brake_on.changed_and(|x| *x == true).await; // predicate version to wait for brake to be on
 
    let send_piezo = signals::PIEZO_MODE.sender();
    send_piezo.send(signals::PiezoModeType::BeepLong);

    let watch_park_brake_on = signals::PARK_BRAKE_ON.sender();
    watch_park_brake_on.send(false);
    signals::send_ble(signals::BleHandles::ParkBrakeOn, &[false as u8]);

    //info!("off: turned parkbrake off");
}


async fn reset() {
    let park_brake_on = signals::PARK_BRAKE_ON.dyn_receiver().unwrap().try_get().unwrap();
    if !park_brake_on {
        signals::PARK_BRAKE_ON.dyn_sender().send(true);
        signals::send_ble(signals::BleHandles::ParkBrakeOn, &[true as u8]);
    }
}