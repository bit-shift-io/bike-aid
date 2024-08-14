use crate::utils::signals;
use embassy_nrf::gpio::{AnyPin, Level, Output, OutputDrive};
use defmt::*;

const TASK_ID: &str = "SWITCH POWER";

#[embassy_executor::task]
pub async fn task(
    pin: AnyPin
) {
    info!("{}: start", TASK_ID);
    let mut sub_button = signals::SWITCH_POWER.subscriber().unwrap();
    let mut pin_state = Output::new(pin, Level::Low, OutputDrive::Standard);

    loop {
        let val = sub_button.next_message_pure().await;
        match val {
            true => {
                pin_state.set_high();
            }
            false => {
                pin_state.set_low();
            }
        }
    }
}