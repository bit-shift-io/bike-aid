use crate::signals;
use embassy_executor::Spawner;
use embassy_nrf::gpio::AnyPin;
use embassy_nrf::gpio::{Input, Pull};
use embassy_time::{Duration, Timer};
use defmt::*;

static TASK_ID : &str = "ALARM";

#[embassy_executor::task]
pub async fn alarm (
    pin : AnyPin
) {
    let INTERVAL = 1000; // 1 sec
    let WARN_INTERVAL = 10; // 10 x 1 sec = 10 sec
    let SENSITIVITY = 40;
    let WARNINGS = 3;

    let mut trigger_count: u8 = 0;
    let mut warn_count = 0;
    let mut warn_interval_count = 0;

    //let pub_hours = signals::CLOCK_HOURS.publisher().unwrap();
    let mut pin_state = Input::new(pin, Pull::Down); // low

    info!("{} : Entering main loop",TASK_ID);
    loop {
        pin_state.wait_for_high().await;
        trigger_count += 1;

        // sensitivity here
        if trigger_count > SENSITIVITY {
            warn_count += 1;
            info!("warn");
        }

        // no warnings
        if warn_count == 0 {
            continue;
        }

        // check the warn count
        if warn_count > WARNINGS {
            //todo: set active alarm, play till user resets
            info!("ALARM!");
        }

        // TODO: need a future joiner thing to continue the count down if there is no motion!
        // give some time before checking again
        info!("delay 1s cooldown");
        Timer::after_millis(INTERVAL).await;
        

        // increment warn interval count
        warn_interval_count += 1;

        // reduce warn count if within time
        if warn_interval_count >= WARN_INTERVAL {
            warn_count -= 1;
            warn_interval_count = 0;
        }

        info!("warn interval:{}", warn_interval_count);
    }
}