use crate::signals;
use embassy_nrf::temp::Temp;
use embassy_time::Timer;
use defmt::*;

static TASK_ID : &str = "TEMPERATURE";

#[embassy_executor::task]
pub async fn temperature (
    mut t : Temp<'static>
) {
    let pub_temperature = signals::TEMPERATURE.publisher().unwrap();

    info!("{} : Entering main loop", TASK_ID);
    loop {
        let value: u16 = t.read().await.to_num::<u16>();
        //info!("{}", v);
        pub_temperature.publish_immediate(value); // in degrees C, no decimals
        Timer::after_secs(60).await;
    }
}