use embassy_time::Timer;
use defmt::*;
use embassy_nrf::gpio::{AnyPin, Level, Output, OutputDrive};
use crate::utils::signals;

const TASK_ID: &str = "LED";

#[embassy_executor::task(pool_size = 2)]
pub async fn task(
    pin: AnyPin,
    id: u8,
) {
    info!("{}", TASK_ID);

    let mut led = Output::new(pin, Level::Low, OutputDrive::Standard);
    let mut led_mode = LedMode::None;
    let mut rec_mode;

    if id == 0 { rec_mode = signals::LED_MODE_WATCH.receiver().unwrap(); }
    else { rec_mode = signals::LED_DEBUG_MODE_WATCH.receiver().unwrap(); }

    loop {
        // Try to poll read new mode
        // doing this way allows us to use the default mode, if no value is set
        if let Some(b) = rec_mode.try_changed() {led_mode = b}

        match led_mode {
            LedMode::None => {
                led.set_low();
                led_mode = rec_mode.changed().await;
            },
            LedMode::Once => {
                single(&mut led).await;
                led_mode = LedMode::None;
            }
            LedMode::Instant => {
                led.set_high();
                Timer::after_millis(5).await;
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
    Instant,
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