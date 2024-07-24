use crate::utils::signals;
use embassy_nrf::gpio::AnyPin;
use embassy_nrf::gpio::{Input, Pull};
use defmt::*;

const TASK_ID: &str = "BRAKE";

#[embassy_executor::task]
pub async fn brake (
    pin: AnyPin
) {
    info!("{}: start", TASK_ID);
    let pub_button = signals::BRAKE_ON.publisher().unwrap();
    let mut pin_state = Input::new(pin, Pull::Up); // high = brake off

    loop {
        pin_state.wait_for_high().await;
        pub_button.publish_immediate(true);

        pin_state.wait_for_low().await;
        pub_button.publish_immediate(false);
    }
}