use crate::utils::signals;
use embassy_nrf::gpio::{AnyPin, Input, Pull};
use defmt::*;

const TASK_ID: &str = "MANUAL OVERRIDE";

#[embassy_executor::task]
pub async fn task(
    pin: AnyPin
) {
    info!("{}: start", TASK_ID);
    // TODO: add request_power or power_state toggle. This task can then handle requests to decide if power is on or off depending on the state
    let pub_button = signals::SWITCH_POWER.publisher().unwrap();
    let mut pin_state = Input::new(pin, Pull::Down); // high = on, low = off

    loop {
        pin_state.wait_for_high().await; // on
        pub_button.publish_immediate(true);

        pin_state.wait_for_low().await; // off
        pub_button.publish_immediate(false);
    }
}