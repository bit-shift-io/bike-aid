//! TWI/i2c - SDA to P0.03, SCL to P0.04

#![no_std]
#![no_main]

// modules/creates
mod system;
mod signals;
mod task_clock;
mod task_twm;
mod task_led;
mod task_manager;
mod task_temperature;
mod task_speed;
mod task_battery;
mod task_alarm;
mod task_throttle;

use embassy_nrf::gpio::Pin;
// imports
use system::System;

// external imports
use embassy_executor::Spawner;
use defmt::*;


#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());

    // Clock Task
    use crate::task_clock::clock;
    spawner.must_spawn(clock());

    // LED Task
    use crate::task_led::led;
    spawner.must_spawn(led(
        p.P0_13.degrade()
    ));


    /*

    // spawn tasks
    spawner.spawn(task_twm::init()).unwrap();
    spawner.spawn(task_clock::init()).unwrap();
    spawner.spawn(task_led::init()).unwrap();
    spawner.spawn(task_manager::init()).unwrap();
    spawner.spawn(task_temperature::init()).unwrap();
    spawner.spawn(task_speed::init()).unwrap();
    spawner.spawn(task_battery::init()).unwrap();
    spawner.spawn(task_alarm::init()).unwrap();
    spawner.spawn(task_throttle::init()).unwrap();
 */
    // test loop
    // loop
    let mut sub_minutes = signals::CLOCK_MINUTES.subscriber().unwrap();
    loop {
        let val = sub_minutes.next_message_pure().await;
        info!("{:02}", val);
    }
 
}