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


pub async fn run() {
    let mut watch = signals::PARK_BRAKE_ON_WATCH.receiver().unwrap();
    let send_power_on = signals::POWER_ON_WATCH.sender();
    let mut state = true;

    loop {
        if let Some(b) = watch.try_get() {state = b}
        
        match state {
            true => {
                let sub_future = watch.changed();
                let task_future = async { 
                    Timer::after_secs(INTERVAL).await;
                    send_power_on.send(false);  // power off
                };
                match select(sub_future, task_future).await {
                    Either::First(val) => { state = val; }
                    Either::Second(_) => {}
                }
            },
            false => { state = watch.changed().await; }
        }
    }
}