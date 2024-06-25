use crate::signals;
use embassy_time::{Duration, Timer};
use defmt::*;

static TASK_ID : &str = "CLOCK";


#[embassy_executor::task]
pub async fn init () {
    let pub_hours = signals::CLOCK_HOURS.publisher().unwrap();
    let pub_minutes = signals::CLOCK_MINUTES.publisher().unwrap();

    let start_time: u64 = embassy_time::Instant::now().as_secs();

    info!("{} : Entering main loop",TASK_ID);
    loop {
        let current_time: u64 = embassy_time::Instant::now().as_secs();
        let all_minutes: u64 = (current_time - start_time) / 60;
        let run_hours: u64 = all_minutes / 60;
        let run_minutes: u64 = all_minutes - (run_hours * 60);
    
        // publish
        pub_minutes.publish_immediate(run_minutes.try_into().unwrap());
        pub_hours.publish_immediate(run_hours.try_into().unwrap());
        Timer::after(Duration::from_secs(60)).await;
    }
}