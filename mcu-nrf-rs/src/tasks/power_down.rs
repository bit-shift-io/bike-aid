use crate::utils::signals;
use defmt::*;
use embassy_time::Timer;
use embassy_futures::select::{select, Either};

const TASK_ID: &str = "POWER DOWN";

#[embassy_executor::task]
pub async fn task() {
    info!("{}: start", TASK_ID);

    //let mut sub = signals::PARK_BRAKE_ON.subscriber().unwrap();
    let mut watch = signals::PARK_BRAKE_ON_WATCH.receiver().unwrap();
    let pub_power = signals::SWITCH_POWER.publisher().unwrap();

    loop {
        let park_brake_on = watch.changed().await;

        match park_brake_on {
            true => {
                let watch_future = watch.changed();
                let task_future = Timer::after_secs(60 * 30); // 30 min timer // power_off_timer();
                match select(watch_future, task_future).await {   
                    Either::First(_) => {} // continue
                    Either::Second(_) => {
                         pub_power.publish_immediate(false);
                     } // power off
                }
            }
            _ => {} // do nothing, continue
        }
    }
}