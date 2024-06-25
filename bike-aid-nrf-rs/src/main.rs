//! Example on how to read a 24C/24LC i2c eeprom.
//!
//! Connect SDA to P0.03, SCL to P0.04

#![no_std]
#![no_main]

// modules
mod system;

// imports
use system::System;

// external imports
use embassy_executor::Spawner;
use embassy_time::Timer;


#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // init system
    System::init();

    /*
    // spawn tasks
    spawner.spawn(task_manager::init()).unwrap();
    spawner.spawn(task_clock::init()).unwrap();
    spawner.spawn(task_temperature::init()).unwrap();
    spawner.spawn(task_speed::init()).unwrap();
    spawner.spawn(task_battery::init()).unwrap();
    spawner.spawn(task_alarm::init()).unwrap();
    spawner.spawn(task_throttle::init()).unwrap();
    */

    // test loop
    loop {
        Timer::after_millis(300).await;
        log::info!("loop");
    }

 
}
