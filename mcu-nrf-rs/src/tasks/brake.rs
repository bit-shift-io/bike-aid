use crate::utils::signals;
use embassy_executor::Spawner;
use embassy_nrf::gpio::AnyPin;
use embassy_nrf::gpio::{Input, Pull};
use defmt::*;
use embassy_futures::select::{select, Either};
use embassy_time::Timer;

const TASK_ID: &str = "BRAKE";
const NO_THROTTLE_THRESHOLD: u16 = 1100;
const MAX_COUNT: u16 = 30 * 10; // this equals 30 seonds of throttle updates

#[embassy_executor::task]
pub async fn task(
    spawner: Spawner,
    pin: AnyPin
) {
    info!("{}: start", TASK_ID);

    // spawn sub tasks
    spawner.must_spawn(brake(pin));

    // power on/off
    let mut sub = signals::SWITCH_POWER.subscriber().unwrap();
    let mut state = false;

    loop { 
        if let Some(b) = sub.try_next_message_pure() {state = b}
        match state {
            true => {
                let sub_future = sub.next_message_pure();
                let task_future = park_brake();
                match select(sub_future, task_future).await {
                    Either::First(val) => { state = val; }
                    Either::Second(_) => { Timer::after_secs(60).await; } // retry
                }
            },
            false => { state = sub.next_message_pure().await; }
        }
    }
}


// TODO: move this to task_future using a join? as the beep can play when turning off with the park brake on
// very unimportant
#[embassy_executor::task]
async fn brake(pin: AnyPin) {
    let pub_brake_on = signals::BRAKE_ON.publisher().unwrap();
    let mut pin_state = Input::new(pin, Pull::None); // high = brake off, low = brake on

    loop {
        pin_state.wait_for_high().await; // brake off
        *signals::BRAKE_ON_MUTEX.lock().await = false;
        pub_brake_on.publish_immediate(false);
        //info!("{}: brake off", TASK_ID);

        pin_state.wait_for_low().await; // brake on
        *signals::BRAKE_ON_MUTEX.lock().await = true;
        pub_brake_on.publish_immediate(true);
        park_brake_off().await;
        //info!("{}: brake on", TASK_ID);
    }
}


async fn park_brake_off() {
    // only send off signal if park brake is on
    let park_brake_on = *signals::PARK_BRAKE_ON_MUTEX.lock().await;
    if park_brake_on {  
        let pub_piezo = signals::PIEZO_MODE.publisher().unwrap();
        pub_piezo.publish_immediate(signals::PiezoModeType::BeepLong);
        *signals::PARK_BRAKE_ON_MUTEX.lock().await = false;
        let pub_park_brake_on = signals::PARK_BRAKE_ON.publisher().unwrap();
        pub_park_brake_on.publish_immediate(false);
        //info!("parkbrake off");
    }
}


async fn park_brake() {
    let pub_piezo = signals::PIEZO_MODE.publisher().unwrap();
    let pub_park_brake_on = signals::PARK_BRAKE_ON.publisher().unwrap();
    let mut sub_throttle = signals::THROTTLE_IN.subscriber().unwrap();
    let mut count = 0;
    *signals::PARK_BRAKE_ON_MUTEX.lock().await = true; // reset/initial state

    loop {
        let throttle_voltage = sub_throttle.next_message_pure().await; // millivolts

        // skip if cruise on or park brake on
        let park_brake_on = { *signals::PARK_BRAKE_ON_MUTEX.lock().await };
        let cruise_on = { *signals::CRUISE_LEVEL.lock().await != 0 };
        //let brake_on = { *signals::BRAKE_ON_MUTEX.lock().await };
        //info!("{} {} {} {}", TASK_ID, cruise_on, brake_on, park_brake_on);
        if cruise_on || park_brake_on { continue; } // brake_on

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