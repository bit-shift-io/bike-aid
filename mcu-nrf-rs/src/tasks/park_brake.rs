use crate::utils::signals;
use defmt::*;
use embassy_futures::select::{select, Either};
use embassy_time::Timer;

const TASK_ID: &str = "PARK BRAKE";
const NO_THROTTLE_THRESHOLD: u16 = 1100;
const MAX_COUNT: u16 = 30 * 10; // this equals 30 seonds of throttle updates

#[embassy_executor::task]
pub async fn task() {
    info!("{}", TASK_ID);

    let mut rec = signals::POWER_ON_WATCH.receiver().unwrap();
    let mut state = false;

    loop { 
        if let Some(b) = rec.try_get() {state = b}
        
        match state {
            true => {
                let watch_future = rec.changed();
                let task_future = run();
                match select(watch_future, task_future).await {
                    Either::First(val) => { state = val; }
                    Either::Second(_) => { Timer::after_secs(60).await; } // retry
                }
            },
            false => { state = rec.changed().await; }
        }
    }
}


// TODO: chain cruise next...

async fn run() {
    let mut watch = signals::PARK_BRAKE_ON_WATCH.receiver().unwrap();
    let mut state = true;

    loop {
        if let Some(b) = watch.try_get() {state = b}
        match state {
            true => { park_brake_off().await; },
            false => { park_brake_on().await; }
        }
    }
}


async fn park_brake_on() {
    // detect when to turn park brake on
    let send_piezo = signals::PIEZO_MODE_WATCH.sender();
    let watch_park_brake_on = signals::PARK_BRAKE_ON_WATCH.sender();
    let mut rec_throttle = signals::THROTTLE_IN_WATCH.receiver().unwrap();
    let mut rec_cruise_level = signals::CRUISE_LEVEL_WATCH.receiver().unwrap();
    let mut count = 0;

    loop {
        let throttle_voltage = rec_throttle.changed().await; // millivolts

        // TODO: chain cruise here to disable instead of in the loop
        let cruise_on = rec_cruise_level.try_get().unwrap() != 0;
        if cruise_on { continue; };

        // detect park brake on
        if throttle_voltage < NO_THROTTLE_THRESHOLD {
            count += 1;

            if count > MAX_COUNT {
                send_piezo.send(signals::PiezoModeType::BeepLong);
                watch_park_brake_on.send(true);
                signals::send_ble(signals::BleHandles::ParkBrakeOn, &(true as u8).to_le_bytes()).await;
                //info!("on: park brake on");
                return;
            }
        } else {
            count = 0;
        }
    }
}


async fn park_brake_off() {
    // wait for brake to be on
    let mut watch_brake_on = signals::BRAKE_ON_WATCH.receiver().unwrap();
    let _ = watch_brake_on.changed_and(|x| *x == true).await; // predicate version to wait for brake to be on
 
    let send_piezo = signals::PIEZO_MODE_WATCH.sender();
    send_piezo.send(signals::PiezoModeType::BeepLong);

    let watch_park_brake_on = signals::PARK_BRAKE_ON_WATCH.sender();
    watch_park_brake_on.send(false);
    signals::send_ble(signals::BleHandles::ParkBrakeOn, &(false as u8).to_le_bytes()).await;

    //info!("off: turned parkbrake off");
}