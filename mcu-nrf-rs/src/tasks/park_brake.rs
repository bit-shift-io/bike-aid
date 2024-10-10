use crate::utils::signals;
use defmt::*;
use embassy_futures::select::{select, Either};
use embassy_time::Timer;

const TASK_ID: &str = "PARK BRAKE";
const NO_THROTTLE_THRESHOLD: u16 = 1100;
const MAX_COUNT: u16 = 30 * 10; // this equals 30 seonds of throttle updates

#[embassy_executor::task]
pub async fn task() {
    info!("{}: start", TASK_ID);

    let mut sub = signals::SWITCH_POWER.subscriber().unwrap();
    let mut state = false;

    loop { 
        if let Some(b) = sub.try_next_message_pure() {state = b}
        match state {
            true => {
                let sub_future = sub.next_message_pure();
                let task_future = run();
                match select(sub_future, task_future).await {
                    Either::First(val) => { state = val; }
                    Either::Second(_) => { Timer::after_secs(60).await; } // retry
                }
            },
            false => { state = sub.next_message_pure().await; }
        }
    }
}


async fn run() {
    let pub_piezo = signals::PIEZO_MODE.publisher().unwrap();
    let pub_park_brake_on = signals::PARK_BRAKE_ON.publisher().unwrap();
    let mut sub_throttle = signals::THROTTLE_IN.subscriber().unwrap();
    let mut count = 0;
    *signals::PARK_BRAKE_ON_MUTEX.lock().await = true; // reset/initial state

    loop {
        let throttle_voltage = sub_throttle.next_message_pure().await; // millivolts

        // TODO: chain parkbrake & cruise here to disable instead of in the loop
        // if cruise on or park brake on
        let park_brake_on = { *signals::PARK_BRAKE_ON_MUTEX.lock().await };
        let cruise_on = { *signals::CRUISE_LEVEL.lock().await != 0 };

        if cruise_on || park_brake_on { 
            continue;
        }
   

        // detect park brake on
        if throttle_voltage < NO_THROTTLE_THRESHOLD {
            count += 1;

            if count > MAX_COUNT {
                count = 0;
                pub_piezo.publish_immediate(signals::PiezoModeType::BeepLong);
                //info!("park brake on");
                *signals::PARK_BRAKE_ON_MUTEX.lock().await = true;
                pub_park_brake_on.publish_immediate(true);
            }
        } else {
            count = 0;
        }
    }
}


async fn park_brake_off() {
    let mut sub_brake_on = signals::BRAKE_ON.subscriber().unwrap();

    loop {
        let brake_on = sub_brake_on.next_message_pure().await;
        if !brake_on { continue; }

        // assume park brake is on at this point...

        // only send off signal if park brake is on
        //let park_brake_on = *signals::PARK_BRAKE_ON_MUTEX.lock().await;
        //if park_brake_on {  
        let pub_piezo = signals::PIEZO_MODE.publisher().unwrap();
        pub_piezo.publish_immediate(signals::PiezoModeType::BeepLong);
        *signals::PARK_BRAKE_ON_MUTEX.lock().await = false;
        let pub_park_brake_on = signals::PARK_BRAKE_ON.publisher().unwrap();
        pub_park_brake_on.publish_immediate(false);
            //info!("parkbrake off");
        //}
        return;
    }
}
