use crate::utils::signals;
use embassy_futures::select::{select, Either};
use embassy_time::Timer;
use defmt::info;

const TASK_ID: &str = "CLOCK";

#[embassy_executor::task]
pub async fn task() {
    info!("{}", TASK_ID);

    let mut rec = signals::POWER_ON.receiver().unwrap();
    let mut state = false;

    loop { 
        if let Some(b) = rec.try_get() {state = b}
        match state {
            true => {
                let watch_future = rec.changed();
                let task_future = run();
                match select(watch_future, task_future).await {
                    Either::First(val) => { state = val; }
                    Either::Second(_) => {} // other task will never end
                }
            },
            false => { state = rec.changed().await; }
        }
    }
}


async fn run() {
    let mut minutes: u8 = 0;
    let mut hours: u8 = 0;
   
    loop {
        Timer::after_secs(60).await;

        // Increment minutes
        minutes = (minutes + 1) % 60;
        signals::send_ble(signals::BleHandles::ClockMinutes, minutes.to_le_bytes().as_slice()).await;
    
        // Increment hours if minutes rolled over to 0
        if minutes == 0 {
            hours = (hours + 1) % 24;
            signals::send_ble(signals::BleHandles::ClockHours, hours.to_le_bytes().as_slice()).await;
        }
    }
}