use crate::utils::signals;
use defmt::info;
use embassy_futures::select::select;

const TASK_ID: &str = "PARK BRAKE";
const NO_THROTTLE_THRESHOLD: u16 = 1100;
const MAX_COUNT: u16 = 30 * 10; // this equals 30 seonds of throttle updates

#[embassy_executor::task]
pub async fn task() {
    info!("{}", TASK_ID);

    let mut rec = signals::POWER_ON.receiver().unwrap();

    loop { 
        if rec.changed().await {
            //info!("{}: power on", TASK_ID);
            let watch_future = rec.changed();
            let task_future = cruise();
            select(watch_future, task_future).await;
            stop().await;
        }
    }
}


pub async fn cruise() {
    // this requires try_get as the state doesnt change by default
    let mut rec = signals::CRUISE_LEVEL.receiver().unwrap();
    let mut state = 0;

    loop { 
        if let Some(b) = rec.try_get() {state = b}
        
        match state {
            0 => {
                //info!("{}: cruise", TASK_ID);
                let watch_future = rec.changed();
                let task_future = park_brake();
                select(watch_future, task_future).await;
            },
            _ => { state = rec.changed().await;}
        }
    }
}


async fn park_brake() {
    // requires try_get as the state doesnt change by default
    let mut watch = signals::PARK_BRAKE_ON.receiver().unwrap();
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


async fn park_brake_off() {
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


async fn stop() {
    let park_brake_on = signals::PARK_BRAKE_ON.dyn_receiver().unwrap().try_get().unwrap();
    if !park_brake_on {
        signals::PARK_BRAKE_ON.dyn_sender().send(true);
        signals::send_ble(signals::BleHandles::ParkBrakeOn, &[true as u8]);
    }
}