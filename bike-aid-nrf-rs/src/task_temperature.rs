use crate::signals;
use embassy_time::{Duration, Timer};
use defmt::*;

static TASK_ID : &str = "TEMPERATURE";


#[embassy_executor::task]
pub async fn temperature () {
    let pub_temperature = signals::TEMPERATURE.publisher().unwrap();

    info!("{} : Entering main loop",TASK_ID);
    loop {
        // todo: read temperature

        // publish
        pub_temperature.publish_immediate(186); // multiply by 0.1 to get C
        Timer::after(Duration::from_secs(60)).await;
    }
}