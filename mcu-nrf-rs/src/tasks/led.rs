use embassy_futures::select::{select, Either};
use embassy_time::Timer;
use defmt::*;
use embassy_nrf::gpio::{AnyPin, Level, Output, OutputDrive};
use crate::utils::signals;

const TASK_ID: &str = "LED";

#[embassy_executor::task]
pub async fn led(
    pin: AnyPin
) {
    info!("{}: start", TASK_ID);
    let mut led = Output::new(pin, Level::Low, OutputDrive::Standard);

    let mut sub_power = signals::SWITCH_POWER.subscriber().unwrap();
    let mut power_state = false;

    loop { 
        if let Some(b) = sub_power.try_next_message_pure() {power_state = b}
        match power_state {
            true => {
                let power_future = sub_power.next_message_pure();
                let task_future = run(&mut led);
                match select(power_future, task_future).await {
                    Either::First(val) => { power_state = val; }
                    Either::Second(_) => {} // other task will never end
                }
            },
            false => { power_state = sub_power.next_message_pure().await; }
        }
    }
}


async fn run<'a>(mut led: &mut Output<'a>) {
    let mut sub_mode = signals::LED_MODE.subscriber().unwrap();
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


#[allow(dead_code)]
#[derive(Clone,Copy)]
pub enum LedMode {
    None,
    Single,
    Double,
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