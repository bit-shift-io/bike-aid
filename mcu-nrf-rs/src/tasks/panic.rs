use crate::utils::signals;
use embassy_futures::select::{select, Either};
use embassy_time::Timer;
use defmt::*;

const TASK_ID: &str = "PANIC";

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
                    Either::Second(_) => { state = false } // switch off task
                }
            },
            false => { state = rec.changed().await; }
        }
    }
}


async fn run() {
    Timer::after_secs(1).await;

    if let Some(msg) = panic_persist::get_panic_message_utf8() {
        info!("{}: {}", TASK_ID, msg);
        signals::send_ble(signals::BleHandles::Uart, msg.as_bytes()).await;
    }

    
    // panic test
    //Timer::after_secs(10).await;
    //defmt::panic!("test panic");
}