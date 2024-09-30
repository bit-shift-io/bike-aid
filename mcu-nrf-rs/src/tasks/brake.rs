use crate::utils::signals;
use embassy_executor::Spawner;
use embassy_nrf::gpio::AnyPin;
use embassy_nrf::gpio::{Input, Pull};
use defmt::*;
use embassy_time::{Duration, Instant, Timer};

const TASK_ID: &str = "BRAKE";
const NO_THROTTLE_THRESHOLD: u16 = 1100;
const MAX_COUNT: u8 = 5 * 10; // this equals 5 seonds of throttle updates

#[embassy_executor::task]
pub async fn task(
    spawner: Spawner,
    pin: AnyPin
) {
    info!("{}: start", TASK_ID);

    // spawn sub tasks
    spawner.must_spawn(park_brake());
    
    let pub_button = signals::BRAKE_ON.publisher().unwrap();
    let mut pin_state = Input::new(pin, Pull::None); // high = brake off, low = brake on

    loop {
        pin_state.wait_for_high().await; // brake off
        pub_button.publish_immediate(false);
        //info!("{}: brake off", TASK_ID);

        pin_state.wait_for_low().await; // brake on
        pub_button.publish_immediate(true);
        //info!("{}: brake on", TASK_ID);
    }
}


#[embassy_executor::task]
async fn park_brake() {
    let mut sub_throttle = signals::THROTTLE_IN.subscriber().unwrap();
    let mut count = 0;

    loop {
        let throttle_voltage = sub_throttle.next_message_pure().await; // millivolts

        if throttle_voltage < NO_THROTTLE_THRESHOLD {
            count += 1;

            if count > MAX_COUNT {
                info!("{} park brake on", TASK_ID);
            }
        } else {
            count = 0;
        }
    }
}