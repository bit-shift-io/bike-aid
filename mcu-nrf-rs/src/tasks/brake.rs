use crate::utils::signals;
use embassy_nrf::gpio::AnyPin;
use embassy_nrf::gpio::{Input, Pull};
use defmt::*;

const TASK_ID: &str = "BRAKE";

#[embassy_executor::task]
pub async fn task(
    pin: AnyPin
) {
    info!("{}: start", TASK_ID);
    let pub_button = signals::BRAKE_ON.publisher().unwrap();
    let mut pin_state = Input::new(pin, Pull::None); // high = brake off, low = brake on

    loop {
        pin_state.wait_for_high().await; // brake off
        pub_button.publish_immediate(false);
        //info!("{}: brake off", TASK_ID);

        pin_state.wait_for_low().await; // brake on
        pub_button.publish_immediate(true);
        //info!("{}: brake on", TASK_ID);
    }
}