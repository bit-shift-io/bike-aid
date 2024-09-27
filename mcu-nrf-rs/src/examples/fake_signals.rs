use crate::utils::{functions, signals};
use defmt::*;
use embassy_time::Timer;

const TASK_ID : &str = "FAKE SIGNALS";
const INTERVAL: u64 = 5000;

#[embassy_executor::task]
pub async fn task () {
    info!("{}: start", TASK_ID);

    // change here for testing
    let pub_test = signals::CLOCK_HOURS.publisher().unwrap();

    let mut count = 7;

    loop {
        //count += 1;
        Timer::after_millis(INTERVAL).await;

        info!("{}: {}", TASK_ID, count);
        pub_test.publish_immediate(count);

    }
}