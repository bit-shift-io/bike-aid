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
    let send_led = signals::LED_MODE_WATCH.sender();
    let send_piezo = signals::PIEZO_MODE_WATCH.sender();

    loop {
        let val = rec_power_on.changed().await;
        
        match val {
            true => {
                pin_state.set_high();
                send_led.send(signals::LedModeType::None);
                send_piezo.send(signals::PiezoModeType::PowerOn);
            }
            false => {
                pin_state.set_low();
                send_led.send(signals::LedModeType::SingleSlow);
                send_piezo.send(signals::PiezoModeType::PowerOff);
            }
        }
    }
}