use crate::utils::signals;
use embassy_nrf::gpio::{AnyPin, Level, Output, OutputDrive};
use defmt::*;

const TASK_ID: &str = "SWITCH POWER";

#[embassy_executor::task]
pub async fn task(
    pin: AnyPin
) {
    info!("{}", TASK_ID);

    let mut rec_power_on = signals::POWER_ON_WATCH.receiver().unwrap();
    let mut pin_state = Output::new(pin, Level::Low, OutputDrive::Standard);
    let pub_led = signals::LED_MODE.publisher().unwrap();
    let pub_piezo = signals::PIEZO_MODE.publisher().unwrap();

    loop {
        let val = rec_power_on.changed().await;
        
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