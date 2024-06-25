//! TWI/i2c - SDA to P0.03, SCL to P0.04

#![no_std]
#![no_main]

// modules/creates
mod system;
mod signals;
mod task_clock;
mod task_twm;
mod task_led;

// imports
use system::System;

// external imports
use embassy_executor::Spawner;
use defmt::*;


#[embassy_executor::main]
async fn main(spawner: Spawner) {
    // init system
    System::init();

   
    // spawn tasks
    spawner.spawn(task_twm::init()).unwrap();
    spawner.spawn(task_clock::init()).unwrap();
    spawner.spawn(task_led::init()).unwrap();
     /*
    spawner.spawn(task_manager::init()).unwrap();
    spawner.spawn(task_clock::init()).unwrap();
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