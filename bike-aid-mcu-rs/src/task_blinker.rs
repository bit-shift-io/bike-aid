//use embassy_rp::gpio::{Output, AnyPin, Level};
use embassy_time::{Duration, Timer};
use esp32c3_hal::{gpio::{AnyPin, Output, IO}, peripherals::Peripherals, prelude::_embedded_hal_digital_v2_OutputPin};
use crate::signals;


#[embassy_executor::task]
pub async fn blinker(
    mut in_blinker_mode : signals::BlinkerModeSub,
    pin : AnyPin<Output<esp32c3_hal::gpio::PushPull>>
) {

    let mut blinker_mode = BlinkerMode::None;

    log::info!("blinker task started");

    let mut led: AnyPin<Output<esp32c3_hal::gpio::PushPull>> = pin;
    led.set_low();

    //let mut led = Output::new(pin,Level::Low);
    loop { 

        // Try to read new blinker mode
        if let Some(b) = in_blinker_mode.try_next_message_pure() {blinker_mode = b}
        log::info!("blinker loop");
        match blinker_mode {
            BlinkerMode::None => {
                //led.set_low();
                led.set_high(); //.set_output_high(false);
                blinker_mode = in_blinker_mode.next_message_pure().await;
            },
            BlinkerMode::OneFast => one_fast(&mut led).await,
            BlinkerMode::TwoFast => two_fast(&mut led).await,
            BlinkerMode::ThreeFast => three_fast(&mut led).await,
            BlinkerMode::OnOffFast => on_off_fast(&mut led).await,
            BlinkerMode::OnOffSlow => on_off_slow(&mut led).await,
        };
    }
}

#[allow(unused)]
async fn one_fast<'a>(led: &mut AnyPin<Output<esp32c3_hal::gpio::PushPull>>) {

    led.set_high(); // Short high
    Timer::after(Duration::from_millis(50)).await;

    led.set_low(); // Long low
    Timer::after(Duration::from_millis(950)).await;
}

#[allow(unused)]
async fn two_fast<'a>(led: &mut AnyPin<Output<esp32c3_hal::gpio::PushPull>>) {
    log::info!("two_fast");
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
async fn three_fast<'a>(led: &mut AnyPin<Output<esp32c3_hal::gpio::PushPull>>) {
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
async fn on_off_fast<'a>(led: &mut AnyPin<Output<esp32c3_hal::gpio::PushPull>>) {
    log::info!("on_off_fast");
    led.set_high(); // Short high
    Timer::after(Duration::from_millis(100)).await;

    led.set_low(); // Medium low
    Timer::after(Duration::from_millis(100)).await;
}

#[allow(unused)]
async fn on_off_slow<'a>(led: &mut AnyPin<Output<esp32c3_hal::gpio::PushPull>>) {
    led.set_high(); // Short high
    Timer::after(Duration::from_millis(100)).await;

    led.set_low(); // Medium low
    Timer::after(Duration::from_millis(100)).await;
}

#[allow(unused)]
#[derive(Clone,Copy)]
pub enum BlinkerMode {
    None,
    OneFast,
    TwoFast,
    ThreeFast,
    OnOffFast,
    OnOffSlow,
}