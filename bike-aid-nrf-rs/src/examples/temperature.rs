use crate::utils::signals;
use embassy_nrf::{peripherals::TEMP, temp::Temp};
use embassy_time::Timer;
use embassy_nrf::{bind_interrupts, interrupt};
use embassy_nrf::interrupt::InterruptExt;
use embassy_nrf::temp;
use defmt::*;

const TASK_ID: &str = "TEMPERATURE";

#[embassy_executor::task]
pub async fn temperature (
    temp: TEMP
) {
    return;
    info!("{}: start", TASK_ID);
    
    bind_interrupts!(struct Irqs {TEMP => temp::InterruptHandler;});
    interrupt::TEMP.set_priority(interrupt::Priority::P3);
    let mut t = Temp::new(temp, Irqs);
    let pub_temperature = signals::TEMPERATURE.publisher().unwrap();

    loop {
        let value: u16 = t.read().await.to_num::<u16>();
        //info!("{}", value);
        pub_temperature.publish_immediate(value); // in degrees C, no decimals
        Timer::after_secs(60).await;
    }
}