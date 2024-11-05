use crate::utils::signals;
use embassy_nrf::gpio::{AnyPin, Level, Output, OutputDrive};
use defmt::info;

const TASK_ID: &str = "SWITCH POWER";

#[embassy_executor::task]
pub async fn task(
    pin: AnyPin
) {
    info!("{}", TASK_ID);

    let mut rec_power_on = signals::POWER_ON.receiver().unwrap();
    let mut pin_state = Output::new(pin, Level::Low, OutputDrive::Standard);
    let send_led = signals::LED_MODE.sender();
    let send_piezo = signals::PIEZO_MODE.sender();
    let mut init = false;

    loop {
        let state = rec_power_on.changed().await;

        match state {
            true => {
                pin_state.set_high();
                signals::send_ble(signals::BleHandles::PowerOn, &[true as u8]).await;
                send_led.send(signals::LedModeType::None);
                send_piezo.send(signals::PiezoModeType::PowerOn);
            }
            false => {
                if init {
                    pin_state.set_low();
                    signals::send_ble(signals::BleHandles::PowerOn, &[false as u8]).await;
                    send_led.send(signals::LedModeType::SingleSlow);
                    send_piezo.send(signals::PiezoModeType::PowerOff);
                } else { init = true; }
            }
        }
    }
}