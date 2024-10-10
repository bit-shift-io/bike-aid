use crate::utils::signals;
use defmt::*;
use embassy_time::Timer;
use embassy_futures::select::{select, Either};

const TASK_ID: &str = "POWER DOWN";

#[embassy_executor::task]
pub async fn task() {
    info!("{}: start", TASK_ID);

    let mut sub = signals::PARK_BRAKE_ON.subscriber().unwrap();

    loop {
        let park_brake_on = sub.next_message_pure().await;

        match park_brake_on {
            true => {
                let sub_future = sub.next_message_pure();
                let task_future = Timer::after_secs(60 * 30); // 30 min timer // power_off_timer();
                match select(sub_future, task_future).await {   
                    Either::First(_) => {} // continue
                    Either::Second(_) => {
                         let pub_power = signals::SWITCH_POWER.publisher().unwrap();
                         pub_power.publish_immediate(false);
                     } // power off
                }
            }
            _ => {} // do nothing, continue
        }
    }
}


// async fn power_off_timer() {
//     // monitor throttle output (not input)
//     let mut sub = signals::THROTTLE_OUT.subscriber().unwrap(); // mv
//     let throttle_threshold = 1200_u16;
//     let mut state = 0;

//     loop { 
//         if let Some(b) = sub.try_next_message_pure() {state = b}
//         match state {
//             state if state < throttle_threshold => {
//                 let sub_future = sub.next_message_pure();
//                 let task_future = Timer::after_secs(60 * 30); // 30 min timer
//                 match select(sub_future, task_future).await {
//                     Either::First(_val) => {} // continue
//                     Either::Second(_) => {
//                         let pub_power = signals::SWITCH_POWER.publisher().unwrap();
//                         pub_power.publish_immediate(false);
//                     } // power off
//                 }
//             },
//             _ => { state = sub.next_message_pure().await; }
//         }
//     }
// }


// async fn power_off_timer() {
//     // Monitor throttle output (not input)
//     let mut sub = signals::THROTTLE_OUT.subscriber().unwrap(); // mv
//     let throttle_threshold = 1200_u16;
//     let mut last_update_time = Instant::now();
//     let inactive_duration = Duration::from_secs(30 * 60); // 30 minutes

//     loop {
//         // Check for the next throttle message
//         if let Some(b) = sub.try_next_message_pure() {
//             // Update the state and reset the last update time
//             last_update_time = Instant::now();
//             if b >= throttle_threshold {
//                 // If the throttle is active, we can skip the timer logic
//                 continue;
//             }
//         }

//         // Check if the timer should trigger
//         if last_update_time.elapsed() >= inactive_duration {
//             // If 30 minutes have passed since the last valid update
//             let pub_power = signals::SWITCH_POWER.publisher().unwrap();
//             pub_power.publish_immediate(false); // Power off
//             break; // Exit the loop after power off
//         }

//         // Wait for a short duration before checking again
//         Timer::after_millis(1000).await;
//     }
// }