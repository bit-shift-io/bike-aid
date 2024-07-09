use crate::signals;
use embassy_time::{Duration, Timer};
use defmt::*;

const TASK_ID: &str = "BATTERY";

#[embassy_executor::task]
pub async fn battery () {
    info!("{}: start", TASK_ID);
    /*
    //let pub_hours = signals::CLOCK_HOURS.publisher().unwrap();

    log::info!("{} : Entering main loop",TASK_ID);
    loop {
    }
     */
}