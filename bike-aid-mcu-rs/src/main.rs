#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

// imports
use embassy_executor::Spawner;
use esp32c3_hal::prelude::*;
use esp_backtrace as _;

// modules
mod signals;
mod task_manager;
mod task_clock;
mod task_speed;
mod system;

use task_manager::TaskManager;



#[main]
async fn main(spawner: Spawner) {
    // init system
    system::init();

    // init task manager
    let mut task_manager: TaskManager = TaskManager::new(spawner);

    // spawn tasks
    spawner.spawn(task_clock::clock()).unwrap();
    spawner.spawn(task_speed::speed()).unwrap();

    // loop
    let mut sub_minutes = signals::CLOCK_MINUTES.subscriber().unwrap();
    loop {
        let val = sub_minutes.next_message_pure().await;
        log::info!("{:02}", val);
    }
}
