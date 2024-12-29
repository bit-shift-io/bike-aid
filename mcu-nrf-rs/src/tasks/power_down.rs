use crate::utils::signals;
use defmt::info;
use embassy_time::Timer;
use embassy_futures::select::{select3, Either3};
use core::future;

const TASK_ID: &str = "POWER DOWN";
const INTERVAL: u64 = 10 * 60; // seconds - 10 mins

#[embassy_executor::task]
pub async fn task() {
    info!("{}", TASK_ID);

    let mut rec_power_on = signals::POWER_ON.receiver().unwrap();
    let mut state_power_on = rec_power_on.try_get().unwrap();

    let mut rec_park_brake_on = signals::PARK_BRAKE_ON.receiver().unwrap();
    let mut state_park_brake_on = rec_park_brake_on.try_get().unwrap();

    loop {
        match select3(rec_power_on.changed(), rec_park_brake_on.changed(), run(state_power_on, state_park_brake_on)).await {
            Either3::First(b) => { state_power_on = b; },
            Either3::Second(b) => { state_park_brake_on = b;},
            Either3::Third(_) => {}
        }
    }
}


pub async fn run(power_on: bool, park_brake_on: bool) {
    if power_on && park_brake_on { power_down_timer().await }
    future::pending().await // wait/yield forever doing nothing
}


pub async fn power_down_timer() {
    Timer::after_secs(INTERVAL).await;
    cortex_m::peripheral::SCB::sys_reset(); //reboot instead
    //signals::REQUEST_POWER_ON.sender().send(false);  // power off
}