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
    let mut brake_input = Input::new(pin, Pull::None); // high = brake off, low = brake on

    // need to turn off the brake when power if off, so that it doesnt rest the handbrake when power comes back on
    // power on/off
    let mut rec_power_on = signals::POWER_ON.receiver().unwrap();

    loop { 
        if rec_power_on.changed().await {
            select(rec_power_on.changed(), brake(&mut brake_input)).await;
            stop().await;
        }
    }
}


pub async fn brake<'a>(brake_input: &mut Input<'a>) {
    let watch_brake_on = signals::BRAKE_ON.sender();
   
    loop {
        brake_input.wait_for_high().await; // brake off
        watch_brake_on.send(false);
        signals::send_ble(signals::BleHandles::BrakeOn, &[false as u8]);
        //info!("{}: off", TASK_ID);

        brake_input.wait_for_low().await; // brake on
        watch_brake_on.send(true);
        signals::send_ble(signals::BleHandles::BrakeOn, &[true as u8]);
        //info!("{}: on", TASK_ID);
    }
}


async fn stop() {
    let brake_on = signals::BRAKE_ON.dyn_receiver().unwrap().try_get().unwrap();
    if brake_on {
        signals::BRAKE_ON.dyn_sender().send(false);
        signals::send_ble(signals::BleHandles::BrakeOn, &[false as u8]);
    }
}