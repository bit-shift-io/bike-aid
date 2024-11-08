use crate::utils::signals;
use embassy_futures::select::{select, Either};
use embassy_time::{Instant, Timer};
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