use crate::signals;
use defmt::*;
use embassy_time::Timer;

const TASK_ID : &str = "FAKE SIGNALS";

#[embassy_executor::task]
pub async fn debug_signals () {
    info!("{}: start", TASK_ID);

    // change pub or sub here for testing
    let pub_throttle = signals::THROTTLE_IN.publisher().unwrap();

    loop {
        Timer::after_millis(1000).await;
        // assign value here for testing (or random)
        let value = 1003;
        info!("{}: {}", TASK_ID, value);
        pub_throttle.publish_immediate(value);
    }
}