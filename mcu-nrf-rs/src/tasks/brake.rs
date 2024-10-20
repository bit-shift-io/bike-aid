use crate::utils::signals;
use embassy_nrf::gpio::{AnyPin, Input, Pull};
use defmt::*;

const TASK_ID: &str = "BRAKE";

#[embassy_executor::task]
pub async fn task(
    pin: AnyPin
) {
    info!("{}", TASK_ID);

    let mut pin_state = Input::new(pin, Pull::None); // high = brake off, low = brake on
    let watch_brake_on = signals::BRAKE_ON_WATCH.sender();
   
    loop {
        pin_state.wait_for_high().await; // brake off
        watch_brake_on.send(false);
        signals::send_ble(signals::BleHandles::BrakeOn, &(false as u8).to_le_bytes()).await;
        //info!("{}: brake off", TASK_ID);

        pin_state.wait_for_low().await; // brake on
        watch_brake_on.send(true);
        signals::send_ble(signals::BleHandles::BrakeOn, &(true as u8).to_le_bytes()).await;
        //info!("{}: brake on", TASK_ID);
    }
}