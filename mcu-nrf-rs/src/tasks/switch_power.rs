use crate::utils::signals;
use embassy_nrf::gpio::{AnyPin, Level, Output, OutputDrive};
use defmt::*;

const TASK_ID: &str = "SWITCH POWER";

#[embassy_executor::task]
pub async fn task(
    pin: AnyPin
) {
    info!("{}: start", TASK_ID);
    let mut sub_power = signals::SWITCH_POWER.subscriber().unwrap();
    let mut pin_state = Output::new(pin, Level::Low, OutputDrive::Standard);
    let pub_led = signals::LED_MODE.publisher().unwrap();
    let pub_piezo = signals::PIEZO_MODE.publisher().unwrap();

    loop {
        let val = sub_power.next_message_pure().await;
        
        match val {
            true => {
                pin_state.set_high();
                pub_led.publish_immediate(signals::LedModeType::None);
                pub_piezo.publish_immediate(signals::PiezoModeType::PowerOn);
            }
            false => {
                pin_state.set_low();
                pub_led.publish_immediate(signals::LedModeType::SingleSlow);
                pub_piezo.publish_immediate(signals::PiezoModeType::PowerOff);
            }
        }
    }
}