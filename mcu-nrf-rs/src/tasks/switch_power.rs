use crate::utils::signals;
use embassy_nrf::gpio::{AnyPin, Level, Output, OutputDrive};
use defmt::info;
use embassy_time::Timer;

const TASK_ID: &str = "SWITCH POWER";

#[embassy_executor::task]
pub async fn task(
    pin: AnyPin
) {
    info!("{}", TASK_ID);

    let mut rec_power_on = signals::REQUEST_POWER_ON.receiver().unwrap();
    let mut pin_state = Output::new(pin, Level::Low, OutputDrive::Standard);
    let send_led = signals::LED_MODE.sender();
    let send_piezo = signals::PIEZO_MODE.sender();
    let send_power_on = signals::POWER_ON.sender();

    loop {
        match rec_power_on.changed().await {
            true => {
                pin_state.set_high();
                Timer::after_millis(100).await; // delay to allow device to power
                send_power_on.send(true);
                send_led.send(signals::LedModeType::None);
                send_piezo.send(signals::PiezoModeType::PowerOn);
                signals::send_ble(signals::BleHandles::PowerOn, &[true as u8]);
            }
            false => {
                pin_state.set_low();
                Timer::after_millis(100).await; // delay to allow device to power
                signals::send_ble(signals::BleHandles::PowerOn, &[false as u8]);
                send_led.send(signals::LedModeType::SingleSlow);
                send_piezo.send(signals::PiezoModeType::PowerOff);
                send_power_on.send(false);
            }
        }
    }
}