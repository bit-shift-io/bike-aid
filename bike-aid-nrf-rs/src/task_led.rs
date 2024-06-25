use embassy_time::Timer;
use defmt::*;
use embassy_nrf::gpio::{Level, Output, OutputDrive};

use crate::system::System;

static TASK_ID : &str = "LED";


#[embassy_executor::task]
pub async fn init () {
    let p = System::peripherals;
    let mut led = Output::new(p.P0_01, Level::Low, OutputDrive::Standard);

    info!("{} : Entering main loop",TASK_ID);
    loop {
        led.set_high();
        Timer::after_millis(300).await;
        led.set_low();
        Timer::after_millis(300).await;
    }
}