use crate::utils::signals;
use embassy_futures::select::select;
use embassy_time::{Instant, Timer};
use defmt::info;

const TASK_ID: &str = "CLOCK";

#[embassy_executor::task]
pub async fn task() {
    info!("{}", TASK_ID);

    // power on/off
    let mut rec = signals::POWER_ON.receiver().unwrap();

    loop { 
        if rec.changed().await {
            let watch_future = rec.changed();
            let task_future = clock();
            select(watch_future, task_future).await;
        }
    }
}


async fn clock() {
    let mut last_minutes: u8 = 0;
    let mut last_hours: u8 = 0;
    let start_time = Instant::now();

    loop {
        Timer::after_secs(60).await;

        let seconds = start_time.elapsed().as_secs();
        let hours = ((seconds % 86400) / 3600) as u8; // 3600 seconds in an hour
        let minutes = ((seconds % 3600) / 60) as u8; // 60 seconds in a minute

        if minutes != last_minutes {
            last_minutes = minutes;
            signals::send_ble(signals::BleHandles::ClockMinutes, minutes.to_le_bytes().as_slice());
        };
    
        if hours != last_hours {
            last_hours = hours;
            signals::send_ble(signals::BleHandles::ClockHours, hours.to_le_bytes().as_slice());
        }
    }
}