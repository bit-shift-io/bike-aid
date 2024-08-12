use crate::utils::signals;
use embassy_time::{Timer, Instant};
use defmt::*;
use embassy_futures::select::{select, Either};

const TASK_ID: &str = "CLOCK";

#[embassy_executor::task]
pub async fn clock () {
    info!("{}: start", TASK_ID);
    let pub_hours = signals::CLOCK_HOURS.publisher().unwrap();
    let pub_minutes = signals::CLOCK_MINUTES.publisher().unwrap();
    let mut start_time: u64 = embassy_time::Instant::now().as_secs();
    let mut sub_power = signals::SWITCH_POWER.subscriber().unwrap();
    let mut power_on = false;

    loop {
        // if let Some(b) = sub_power.try_next_message_pure() {power_on = b} // pass b into a value we can use

        // match power_on {
        //     true => {
        //         async {
        //             let current_time: u64 = embassy_time::Instant::now().as_secs();
        //             let all_minutes: u64 = (current_time - start_time) / 60;
        //             let run_hours: u64 = all_minutes / 60;
        //             let run_minutes: u64 = all_minutes - (run_hours * 60);
                
        //             // publish
        //             // todo: dont publish if no change (for hours, minutes always change)
        //             pub_minutes.publish_immediate(run_minutes.try_into().unwrap());
        //             pub_hours.publish_immediate(run_hours.try_into().unwrap());
        //             Timer::after_secs(60).await;
        //         }.await;
        //     },
        //     false => {
        //         info!("{}: power off", TASK_ID);
        //         power_on = sub_power.next_message_pure().await;
        //         start_time = embassy_time::Instant::now().as_secs();
        //     }
        // }



        let power_future = sub_power.next_message_pure();

        let clock_future = async {
            let current_time: u64 = embassy_time::Instant::now().as_secs();
            let all_minutes: u64 = (current_time - start_time) / 60;
            let run_hours: u64 = all_minutes / 60;
            let run_minutes: u64 = all_minutes - (run_hours * 60);
        
            // publish
            pub_minutes.publish_immediate(run_minutes.try_into().unwrap());
            pub_hours.publish_immediate(run_hours.try_into().unwrap());
            Timer::after_secs(60).await;
        };

        match select(power_future, clock_future).await {
            Either::First(val) => {
                if val {
                    info!("POWER turned OFF");
                    // power turned off, wait to power to be on again
                    sub_power.next_message_pure().await;
                } else {
                    // reset time
                    info!("RESET TIME");
                    start_time = embassy_time::Instant::now().as_secs();
                }
            },
            Either::Second(()) => {
                // task here
                info!("{}: tick", TASK_ID);
            },
        }


    }
}