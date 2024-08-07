use embassy_time::{Duration, Timer};
use defmt::*;
use embassy_nrf::gpio::{AnyPin, Level, Output, OutputDrive};
use crate::utils::signals;

const TASK_ID: &str = "LED";

#[embassy_executor::task]
pub async fn led (
    pin: AnyPin
) {
    info!("{}: start", TASK_ID);
    let mut sub_mode = signals::LED_MODE.subscriber().unwrap();
    let mut led = Output::new(pin, Level::Low, OutputDrive::Standard);
    let mut led_mode = LedMode::Double;

    loop { 
        // Try to poll read new mode
        // doing this way allows us to use the default mode, if no value is set
        if let Some(b) = sub_mode.try_next_message_pure() {led_mode = b}

        match led_mode {
            LedMode::None => {
                led.set_low();
                led_mode = sub_mode.next_message_pure().await;
            },
            LedMode::Double => double(&mut led).await,
            LedMode::Single => single(&mut led).await,
        };
    }
}

#[derive(Clone,Copy)]
pub enum LedMode {
    None,
    Single,
    Double,
}


async fn double<'a>(led: &mut Output<'a>) {
    led.set_high(); // Short high
    Timer::after(Duration::from_millis(150)).await;

    led.set_low(); // Long low
    Timer::after(Duration::from_millis(300)).await;

    led.set_high(); // Short high
    Timer::after(Duration::from_millis(150)).await;

    led.set_low(); // Long low
    Timer::after(Duration::from_millis(1500)).await;
}


async fn single<'a>(led: &mut Output<'a>) {
    led.set_high(); // Meium high
    Timer::after(Duration::from_millis(200)).await;

    led.set_low(); // Slow low
    Timer::after(Duration::from_millis(1000)).await;
}