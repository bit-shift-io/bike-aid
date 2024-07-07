use crate::signals;
use embassy_nrf::gpio::{AnyPin, Level, Output, OutputDrive};
use defmt::*;

static TASK_ID : &str = "RELAY POWER";

#[embassy_executor::task]
pub async fn relay_power (
    pin : AnyPin
) {
    let mut sub_button = signals::BUTTON_ON.subscriber().unwrap();
    let mut pin_state = Output::new(pin, Level::Low, OutputDrive::Standard0HighDrive1);

    info!("{} : Entering main loop", TASK_ID);
    loop {
        let val = sub_button.next_message_pure().await;
        info!("{}", val);
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