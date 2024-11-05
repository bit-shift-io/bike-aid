use crate::utils::signals;
use embassy_nrf::gpio::{AnyPin, Input, Pull};
use defmt::info;
use embassy_time::Timer;

const TASK_ID: &str = "MANUAL OVERRIDE";
const INTERVAL: u64 = 500;

#[embassy_executor::task]
pub async fn task(
    pin: AnyPin
) {
    info!("{}", TASK_ID);
    // TODO: add request_power or power_state toggle. This task can then handle requests to decide if power is on or off depending on the state
    let send_power_on = signals::POWER_ON.sender();
    let mut pin_state = Input::new(pin, Pull::Up); // high = off, low = on

    // note: delay due to switch debounce
    loop {
        pin_state.wait_for_high().await; // off
        send_power_on.send(false);
        //info!("{}: off", TASK_ID);
        Timer::after_millis(INTERVAL).await;

        pin_state.wait_for_low().await; // on
        send_power_on.send(true);
        //info!("{}: on", TASK_ID);
        Timer::after_millis(INTERVAL).await;
    }
}