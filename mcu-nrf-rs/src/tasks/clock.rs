use crate::utils::signals;
use embassy_futures::select::{select, Either};
use embassy_time::{Timer, Instant};
use defmt::*;

const TASK_ID: &str = "CLOCK";

#[embassy_executor::task]
pub async fn task() {
    info!("{}", TASK_ID);

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
                    Either::Second(_) => {} // other task will never end
                }
            },
            false => { state = rec.changed().await; }
        }
    }
}


async fn run() {
    let start_time: u64 = Instant::now().as_secs();
    let mut last_hours: u64 = 0;

    loop {
        let current_time: u64 = Instant::now().as_secs();
        let all_minutes: u64 = (current_time - start_time) / 60;
        let run_hours: u64 = all_minutes / 60;
        let run_minutes: u64 = all_minutes - (run_hours * 60);

        // minutes always change
        let minutes: u8 = run_minutes.try_into().unwrap();
        signals::send_ble(signals::BleHandles::ClockMinutes, minutes.to_le_bytes().as_slice());

        // hours if changed
        if last_hours != run_hours {
            last_hours = run_hours;
            let hours: u8 = run_hours.try_into().unwrap();
            signals::send_ble(signals::BleHandles::ClockHours, hours.to_le_bytes().as_slice());
        }

        Timer::after_secs(60).await;
    }
}