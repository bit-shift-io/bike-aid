use crate::signals;
use embassy_time::{Duration, Timer};

static TASK_ID : &str = "THROTTLE";


#[embassy_executor::task]
pub async fn throttle () {
    /*
    //let pub_hours = signals::CLOCK_HOURS.publisher().unwrap();

    log::info!("{} : Entering main loop",TASK_ID);
    loop {
    }
     */
}