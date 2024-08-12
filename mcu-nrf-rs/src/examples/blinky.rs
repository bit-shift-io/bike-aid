use embassy_time::Timer;
use defmt::*;
use embassy_nrf::gpio::{AnyPin, Level, Output, OutputDrive};

const TASK_ID: &str = "BLINKY";

#[embassy_executor::task]
pub async fn blinky (
    pin: AnyPin
) {
    info!("{}: start", TASK_ID);
    let mut led = Output::new(pin, Level::Low, OutputDrive::Standard);

    loop { 
        led.set_high();
        Timer::after_millis(3000).await;
        led.set_low();
        Timer::after_millis(3000).await;
    };
}