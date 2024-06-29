use crate::signals;
use embassy_nrf::saadc::Saadc;
use embassy_time::Timer;
use defmt::*;

static TASK_ID : &str = "THROTTLE";

#[embassy_executor::task]
pub async fn throttle (
    mut saadc: Saadc<'static, 1>,
) {
    let pub_throttle = signals::THROTTLE.publisher().unwrap();

    info!("{} : Entering main loop", TASK_ID);
    loop {
        let mut buf = [0; 1];
        saadc.sample(&mut buf).await;
        pub_throttle.publish_immediate(buf[0]); // TODO: check if these can be negative values, the dac only takes positive values
        info!("sample: {=i16}", &buf[0]);
        Timer::after_millis(100).await;
    }
}