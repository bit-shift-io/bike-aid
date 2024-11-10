use crate::utils::signals;
use defmt::info;
use embassy_executor::Spawner;
use embassy_time::Timer;

const TASK_ID : &str = "SIGNAL TEST";

#[embassy_executor::task]
pub async fn task(spawner: Spawner) {
    info!("{}", TASK_ID);

    // power on
    let send_power_on = signals::REQUEST_POWER_ON.sender();
    Timer::after_millis(1000).await;
    send_power_on.send(true);

    // start sending throttle values
    spawner.must_spawn(throttle());

    // send brake on
    let send_brake_on = signals::BRAKE_ON.sender();
    send_brake_on.send(true);

    // wait for park brake to engage
    Timer::after_secs(32).await;

    // send brake off
    send_brake_on.send(false);

    
    // let mut count = 7;
    // loop {
    //     Timer::after_millis(100).await;

    // }

    info!("{}: end", TASK_ID);
}


#[embassy_executor::task]
pub async fn throttle() {
    let send_throttle_input = signals::THROTTLE_IN.sender();
    let value = 1500u16;

    loop {
        Timer::after_millis(100).await;
        send_throttle_input.send(value);
    }
}