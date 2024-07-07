use crate::signals;
use embassy_nrf::gpio::AnyPin;
use embassy_nrf::gpio::{Input, Pull};
use defmt::*;

static TASK_ID : &str = "BUTTON";

#[embassy_executor::task]
pub async fn button (
    pin : AnyPin
) {
    let pub_button = signals::BUTTON_ON.publisher().unwrap();
    let mut pin_state = Input::new(pin, Pull::Down); // low

    info!("{} : Entering main loop", TASK_ID);
    loop {
        pin_state.wait_for_high().await;
        pub_button.publish_immediate(true);

        pin_state.wait_for_low().await;
        pub_button.publish_immediate(false);
    }
}