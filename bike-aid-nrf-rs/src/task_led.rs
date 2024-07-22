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
    let mut in_led_mode = signals::LED_MODE.subscriber().unwrap();
    let mut led_mode = LedMode::None;
    let mut led = Output::new(pin, Level::Low, OutputDrive::Standard);

    loop { 
        // Try to read new mode
        if let Some(b) = in_led_mode.try_next_message_pure() {led_mode = b}

        match led_mode {
            LedMode::None => {
                led.set_low();
                led_mode = in_led_mode.next_message_pure().await;
            },
            LedMode::OneFast => one_fast(&mut led).await,
            LedMode::TwoFast => two_fast(&mut led).await,
            LedMode::ThreeFast => three_fast(&mut led).await,
            LedMode::OnOffFast => on_off_fast(&mut led).await,
            LedMode::OnOffSlow => on_off_slow(&mut led).await,
        };
    }
}

#[allow(unused)]
#[derive(Clone,Copy)]
pub enum LedMode {
    None,
    OneFast,
    TwoFast,
    ThreeFast,
    OnOffFast,
    OnOffSlow,
}

#[allow(unused)]
async fn one_fast<'a>(led: &mut Output<'a>) {

    led.set_high(); // Short high
    Timer::after(Duration::from_millis(50)).await;

    led.set_low(); // Long low
    Timer::after(Duration::from_millis(950)).await;
}

#[allow(unused)]
async fn two_fast<'a>(led: &mut Output<'a>) {
    led.set_high(); // Short high
    Timer::after(Duration::from_millis(50)).await;

    led.set_low(); // Medium low
    Timer::after(Duration::from_millis(100)).await;

    led.set_high(); // Short high
    Timer::after(Duration::from_millis(50)).await;

    led.set_low(); // Long low
    Timer::after(Duration::from_millis(800)).await;
}

#[allow(unused)]
async fn three_fast<'a>(led: &mut Output<'a>) {
    led.set_high(); // Short high
    Timer::after(Duration::from_millis(50)).await;

    led.set_low(); // Medium low
    Timer::after(Duration::from_millis(100)).await;

    led.set_high(); // Short high
    Timer::after(Duration::from_millis(50)).await;

    led.set_low(); // Medium low
    Timer::after(Duration::from_millis(100)).await;

    led.set_high(); // Short high
    Timer::after(Duration::from_millis(50)).await;

    led.set_low(); // Long low
    Timer::after(Duration::from_millis(650)).await;
}

#[allow(unused)]
async fn on_off_fast<'a>(led: &mut Output<'a>) {
    led.set_high(); // Short high
    Timer::after(Duration::from_millis(100)).await;

    led.set_low(); // Medium low
    Timer::after(Duration::from_millis(200)).await;
}

#[allow(unused)]
async fn on_off_slow<'a>(led: &mut Output<'a>) {
    led.set_high(); // Meium high
    Timer::after(Duration::from_millis(200)).await;

    led.set_low(); // Slow low
    Timer::after(Duration::from_millis(1000)).await;
}