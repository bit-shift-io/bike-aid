use crate::utils::signals;
use embassy_nrf::gpio::{AnyPin, Level, Output, OutputDrive};
use defmt::info;

const TASK_ID: &str = "SWITCH LIGHT";

#[embassy_executor::task]
pub async fn task(
    pin: AnyPin
) {
    info!("{}", TASK_ID);
    let mut rec_button = signals::SWITCH_LIGHT.receiver().unwrap();
    let mut pin_state = Output::new(pin, Level::Low, OutputDrive::Standard);

    loop {
        let val = rec_button.changed().await;
        info!("{}: {}", TASK_ID, val);
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