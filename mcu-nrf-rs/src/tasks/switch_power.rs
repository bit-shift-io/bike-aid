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
    let pub_led = signals::LED_MODE.publisher().unwrap();

    loop {
        let val = sub_button.next_message_pure().await;
        info!("{}: {}", TASK_ID, val);
        match val {
            true => {
                pin_state.set_high();
                pub_led.publish_immediate(signals::LedModeType::Double);
            }
            false => {
                pin_state.set_low();
                pub_led.publish_immediate(signals::LedModeType::None);
            }
        }
    }
}