use crate::utils::signals;
use embassy_nrf::gpio::{AnyPin, Input, Pull};
use defmt::info;
use embassy_futures::select::{select, Either};
use embassy_time::Timer;

const TASK_ID: &str = "BRAKE";

#[embassy_executor::task]
pub async fn task(
    pin: AnyPin
) {
    info!("{}", TASK_ID);

    let mut brake_state = Input::new(pin, Pull::None); // high = brake off, low = brake on
    // need to turn off the brake when power if off, so that it doesnt rest the handbrake when power comes back on
    let mut rec = signals::POWER_ON.receiver().unwrap();
    let mut state = false;

    loop { 
        if let Some(b) = rec.try_get() {state = b}
        
        match state {
            true => {
                let watch_future = rec.changed();
                let task_future = run(&mut brake_state);
                match select(watch_future, task_future).await {
                    Either::First(val) => { state = val; }
                    Either::Second(_) => { Timer::after_secs(60).await; } // retry
                }
            },
            false => { state = rec.changed().await; }
        }
    }
}

pub async fn run<'a>(
    brake_state: &mut Input<'a>
) {
    
    let watch_brake_on = signals::BRAKE_ON.sender();
   
    loop {
        brake_state.wait_for_high().await; // brake off
        watch_brake_on.send(false);
        signals::send_ble(signals::BleHandles::BrakeOn, &(false as u8).to_le_bytes()).await;
        //info!("{}: brake off", TASK_ID);

        brake_state.wait_for_low().await; // brake on
        watch_brake_on.send(true);
        signals::send_ble(signals::BleHandles::BrakeOn, &(true as u8).to_le_bytes()).await;
        //info!("{}: brake on", TASK_ID);
    }
}