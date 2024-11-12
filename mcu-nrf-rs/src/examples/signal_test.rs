use crate::{ble::server::QUEUE_CHANNEL, utils::signals::{self, send_ble, BleHandles}};
use defmt::info;
use embassy_executor::Spawner;
use embassy_futures::select::{select, select3, Either, Either3};
use embassy_time::Timer;

const TASK_ID : &str = "SIGNAL TEST";

#[embassy_executor::task]
pub async fn task(spawner: Spawner) {
    info!("{}", TASK_ID);

    spawner.must_spawn(test_chain());

    // power on
    let send_power_on = signals::REQUEST_POWER_ON.sender();
    Timer::after_millis(1000).await;
    send_power_on.send(true);

    // park brake off
    let send_park_brake_on = signals::PARK_BRAKE_ON.sender();
    Timer::after_millis(1000).await;
    info!("{}: PB off", TASK_ID);
    send_park_brake_on.send(false);

    // park brake on
    let send_park_brake_on = signals::PARK_BRAKE_ON.sender();
    Timer::after_millis(1000).await;
    info!("{}: PB on", TASK_ID);
    send_park_brake_on.send(true);

     // park brake off
     let send_park_brake_on = signals::PARK_BRAKE_ON.sender();
     Timer::after_millis(1000).await;
     info!("{}: PB off", TASK_ID);
     send_park_brake_on.send(false);

    // park brake on
    let send_park_brake_on = signals::PARK_BRAKE_ON.sender();
    Timer::after_millis(1000).await;
    info!("{}: PB on", TASK_ID);
    send_park_brake_on.send(true);

    info!("{}: end", TASK_ID);
}


#[embassy_executor::task]
async fn test_chain() {
    let mut rec_power_on = signals::POWER_ON.receiver().unwrap();
    let mut rec_park_brake_on = signals::PARK_BRAKE_ON.receiver().unwrap();
    let mut rec_cruise = signals::CRUISE_LEVEL.receiver().unwrap();
    let mut state_power_on = rec_power_on.try_get().unwrap();
    let mut state_park_brake_on = rec_park_brake_on.try_get().unwrap();
    let mut state_cruise_level = rec_cruise.try_get().unwrap();

    loop {
        // power on && park brake off
        match select3(rec_power_on.changed(), rec_park_brake_on.changed(), rec_cruise.changed()).await {
            Either3::First(b) => { state_power_on = b; },
            Either3::Second(b) => { state_park_brake_on = b;},
            Either3::Third(b) => { state_cruise_level = b;},
        }

        info!("{}: power on: {}, park brake on: {}", TASK_ID, state_power_on, state_park_brake_on);
    
        if !(state_power_on && !state_park_brake_on && state_cruise_level == 1) {
            continue;
        }

        // do stuff we want here!
        // OR we add all our tasks in the first batch.... then we can check what we should do....?
    }
}



async fn test_queue(spawner: Spawner) {
  send_ble(BleHandles::Temperature, &[18u8]);
  send_ble(BleHandles::Temperature, &[18u8]);
  send_ble(BleHandles::Temperature, &[18u8]);
  send_ble(BleHandles::Temperature, &[18u8]);
  send_ble(BleHandles::Temperature, &[18u8]);

  send_ble(BleHandles::Uart, b"test");
  send_ble(BleHandles::Uart, b"test2");

  let rec_queue = QUEUE_CHANNEL.receiver();
  let size = rec_queue.len();
  info!("{}: queue size: {}", TASK_ID, size);
  
}


async fn test1(spawner: Spawner) {

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