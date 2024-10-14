use embassy_time::Timer;
use defmt::*;
use embassy_nrf::gpio::{AnyPin, Level, Output, OutputDrive};
use crate::utils::signals;

const TASK_ID: &str = "LED";

#[embassy_executor::task]
pub async fn task(
    pin: AnyPin
) {
    info!("{}", TASK_ID);
    let mut led = Output::new(pin, Level::Low, OutputDrive::Standard);
    let mut sub_mode = signals::LED_MODE.subscriber().unwrap();
    let mut led_mode = LedMode::None;

    loop {
        // Try to poll read new mode
        // doing this way allows us to use the default mode, if no value is set
        if let Some(b) = sub_mode.try_next_message_pure() {led_mode = b}

        match led_mode {
            LedMode::None => {
                led.set_low();
                led_mode = sub_mode.next_message_pure().await;
            },
            LedMode::Once => {
                single(&mut led).await;
                led_mode = LedMode::None;
            }
            LedMode::Double => double(&mut led).await, // loop
            LedMode::Single => single(&mut led).await, // loop
            LedMode::SingleSlow => single_slow(&mut led).await, // loop
            LedMode::DoubleSlow => double_slow(&mut led).await, // loop
        };
    }

}


#[allow(dead_code)]
#[derive(Clone,Copy)]
pub enum LedMode {
    None,
    Single,
    Double,
    Once,
    SingleSlow,
    DoubleSlow,
}


async fn single_slow<'a>(led: &mut Output<'a>) {
    led.set_high(); // Short high
    Timer::after_millis(150).await;

    led.set_low(); // Long low
    Timer::after_secs(10).await;
}


async fn double_slow<'a>(led: &mut Output<'a>) {
    led.set_high(); // Short high
    Timer::after_millis(150).await;

    led.set_low(); // Long low
    Timer::after_millis(300).await;

    led.set_high(); // Short high
    Timer::after_millis(150).await;

    led.set_low(); // Long low
    Timer::after_secs(10).await;
}


async fn double<'a>(led: &mut Output<'a>) {
    led.set_high(); // Short high
    Timer::after_millis(150).await;

    led.set_low(); // Long low
    Timer::after_millis(300).await;

    led.set_high(); // Short high
    Timer::after_millis(150).await;

    led.set_low(); // Long low
    Timer::after_millis(1500).await;
}


async fn single<'a>(led: &mut Output<'a>) {
    led.set_high(); // Meium high
    Timer::after_millis(200).await;

    led.set_low(); // Slow low
    Timer::after_millis(1000).await;
}