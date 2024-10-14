use crate::utils::signals;
use defmt::*;
use embassy_time::Timer;
use embassy_futures::select::{select, Either};

const TASK_ID: &str = "POWER DOWN";
const INTERVAL: u64 = 20 * 60; // seconds - 20 mins

#[embassy_executor::task]
pub async fn task() {
    info!("{}", TASK_ID);

    // power on/off
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


pub async fn run() {
    let mut watch = signals::PARK_BRAKE_ON_WATCH.receiver().unwrap();
    let pub_power = signals::SWITCH_POWER.publisher().unwrap();
    let mut state = true;

    loop {
        if let Some(b) = watch.try_get() {state = b}
        
        match state {
            true => {
                let sub_future = watch.changed();
                let task_future = { Timer::after_secs(INTERVAL) };
                match select(sub_future, task_future).await {
                    Either::First(val) => { state = val; }
                    Either::Second(_) => { pub_power.publish_immediate(false); } // power off
                }
            },
            false => { state = watch.changed().await; }
        }
    }
}