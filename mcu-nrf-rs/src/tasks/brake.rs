use crate::utils::signals;
use embassy_nrf::gpio::{AnyPin, Input, Pull};
use defmt::*;

const TASK_ID: &str = "BRAKE";

#[embassy_executor::task]
pub async fn task(
    pin: AnyPin
) {
    info!("{}: start", TASK_ID);

    let mut pin_state = Input::new(pin, Pull::None); // high = brake off, low = brake on
    let pub_brake_on = signals::BRAKE_ON.publisher().unwrap();
    
    // TODO: replace brake mutex/pubsub with watch channel
    
    loop {
        pin_state.wait_for_high().await; // brake off
        *signals::BRAKE_ON_MUTEX.lock().await = false;
        pub_brake_on.publish_immediate(false);
        //info!("{}: brake off", TASK_ID);

        pin_state.wait_for_low().await; // brake on
        *signals::BRAKE_ON_MUTEX.lock().await = true;
        pub_brake_on.publish_immediate(true);
        //info!("{}: brake on", TASK_ID);
    }
}


// #[embassy_executor::task]
// pub async fn task(
//     pin: AnyPin
// ) {
//     info!("{}: start", TASK_ID);

//     let mut pin_state = Input::new(pin, Pull::None); // high = brake off, low = brake on

//     // TODO: i dont think we need the power state?

    
//     // power on/off
//     let mut sub = signals::SWITCH_POWER.subscriber().unwrap();
//     let mut state = false;

//     loop { 
//         if let Some(b) = sub.try_next_message_pure() {state = b}
//         match state {
//             true => {
//                 let sub_future = sub.next_message_pure();
//                 let task_future = run(&mut pin_state);
//                 match select(sub_future, task_future).await {
//                     Either::First(val) => { state = val; }
//                     Either::Second(_) => { Timer::after_secs(60).await; } // retry
//                 }
//             },
//             false => { state = sub.next_message_pure().await; }
//         }
//     }
// }


// async fn run(pin_state: &mut Input<'_>) {
//     let pub_brake_on = signals::BRAKE_ON.publisher().unwrap();
    
//     loop {
//         pin_state.wait_for_high().await; // brake off
//         *signals::BRAKE_ON_MUTEX.lock().await = false;
//         pub_brake_on.publish_immediate(false);
//         //info!("{}: brake off", TASK_ID);

//         pin_state.wait_for_low().await; // brake on
//         *signals::BRAKE_ON_MUTEX.lock().await = true;
//         pub_brake_on.publish_immediate(true);
//         //info!("{}: brake on", TASK_ID);
//     }
// }