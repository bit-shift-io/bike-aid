use crate::utils::signals;
use embassy_nrf::gpio::{AnyPin, Input, Pull};
use defmt::info;
use embassy_futures::select::select;

const TASK_ID: &str = "BRAKE";

#[embassy_executor::task]
pub async fn task(
    pin: AnyPin
) {
    info!("{}", TASK_ID);

    // if brake not plugged in, this will float around and cause issues with things...
    let mut brake_state = Input::new(pin, Pull::None); // high = brake off, low = brake on

    // need to turn off the brake when power if off, so that it doesnt rest the handbrake when power comes back on
    // power on/off
    let mut rec = signals::POWER_ON.receiver().unwrap();

    loop { 
        match rec.changed().await {
            true => {
                let watch_future = rec.changed();
                let task_future = run(&mut brake_state);
                select(watch_future, task_future).await;
            },
            false => {}
        }
    }
}


pub async fn run<'a>(brake_state: &mut Input<'a>) {
    let watch_brake_on = signals::BRAKE_ON.sender();
   
    loop {
        brake_state.wait_for_high().await; // brake off
        watch_brake_on.send(false);
        signals::send_ble(signals::BleHandles::BrakeOn, &(false as u8).to_le_bytes());
        //info!("{}: off", TASK_ID);

        brake_state.wait_for_low().await; // brake on
        watch_brake_on.send(true);
        signals::send_ble(signals::BleHandles::BrakeOn, &(true as u8).to_le_bytes());
        //info!("{}: on", TASK_ID);
    }
}


// async fn stop() {
//     let brake_on = signals::BRAKE_ON.dyn_receiver().unwrap().try_get().unwrap();
//     if brake_on {
//         signals::BRAKE_ON.dyn_sender().send(false);
//         signals::send_ble(signals::BleHandles::BrakeOn, &[false as u8]);
//     }
// }