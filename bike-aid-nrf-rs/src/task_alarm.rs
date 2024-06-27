use crate::signals;
use embassy_time::{Duration, Timer};

static TASK_ID : &str = "ALARM";


#[embassy_executor::task]
pub async fn alarm () {
    /*
    //let pub_hours = signals::CLOCK_HOURS.publisher().unwrap();

    log::info!("{} : Entering main loop",TASK_ID);
    loop {
    }
    */
}