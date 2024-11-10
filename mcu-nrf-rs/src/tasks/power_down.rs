use crate::utils::signals;
use defmt::info;
use embassy_time::Timer;
use embassy_futures::select::select;

const TASK_ID: &str = "POWER DOWN";
const INTERVAL: u64 = 10 * 60; // seconds - 10 mins

#[embassy_executor::task]
pub async fn task() {
    info!("{}", TASK_ID);

    // power on/off
    let mut rec = signals::POWER_ON.receiver().unwrap();

    loop { 
        match rec.changed().await {
            true => {
                let watch_future = rec.changed();
                let task_future = run();
                select(watch_future, task_future).await;
            },
            false => {}
        }
    }
}


pub async fn run() {
    let mut watch = signals::PARK_BRAKE_ON.receiver().unwrap();
    let send_power_on = signals::REQUEST_POWER_ON.sender();

    loop {
        match watch.changed().await {
            true => {
                let rec_future = watch.changed();
                let task_future = async { 
                    Timer::after_secs(INTERVAL).await;
                    send_power_on.send(false);  // power off
                };
                select(rec_future, task_future).await;
            },
            false => {}
        }
    }
}