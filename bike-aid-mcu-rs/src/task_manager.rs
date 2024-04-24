use crate::signals;
use embassy_time::{Duration, Timer};

static TASK_ID : &str = "TASK_MANAGER";

#[embassy_executor::task]
pub async fn init () {
    let pub1 = signals::TEST_CHANNEL.publisher().unwrap();

    log::info!("{} : Entering main loop",TASK_ID);
    loop {
        pub1.publish_immediate(2);
        Timer::after(Duration::from_millis(1000)).await;
    }
}