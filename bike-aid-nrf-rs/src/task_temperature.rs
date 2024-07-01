use crate::signals;
use embassy_nrf::{peripherals::TEMP, temp::Temp};
use embassy_time::Timer;
use defmt::*;

static TASK_ID : &str = "TEMPERATURE";

#[embassy_executor::task]
pub async fn temperature (
    test : TEMP
) {
    use embassy_nrf::{bind_interrupts, temp};
    bind_interrupts!(struct Irqs {
        TEMP => temp::InterruptHandler;
    });
    let mut t = Temp::new(test, Irqs);
    let pub_temperature = signals::TEMPERATURE.publisher().unwrap();

    info!("{} : Entering main loop", TASK_ID);
    loop {
        let value: u16 = t.read().await.to_num::<u16>();
        //info!("{}", value);
        pub_temperature.publish_immediate(value); // in degrees C, no decimals
        Timer::after_secs(60).await;
    }
}