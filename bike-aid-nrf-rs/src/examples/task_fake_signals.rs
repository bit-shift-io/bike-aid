use crate::{functions::*, signals};
use defmt::*;
use embassy_time::Timer;

const TASK_ID : &str = "FAKE SIGNALS";
const INTERVAL: u64 = 5000;

#[embassy_executor::task]
pub async fn debug_signals () {
    info!("{}: start", TASK_ID);

    // change pub or sub here for testing
    let pub_throttle = signals::THROTTLE_IN.publisher().unwrap();
    let pub_uart = signals::UART_WRITE.publisher().unwrap();
    let padded_byte_array = str_to_array("Hello, World!");

    loop {
        Timer::after_millis(INTERVAL).await;
        // assign value here for testing (or random)
        let value = 1003;
        info!("{}: {}", TASK_ID, value);
        pub_throttle.publish_immediate(value);
        pub_uart.publish_immediate(padded_byte_array);
    }
}