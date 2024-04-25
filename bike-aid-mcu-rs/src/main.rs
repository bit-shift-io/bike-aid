#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

// modules
mod signals;
mod task_manager;
mod task_clock;
mod task_speed;
mod system;
mod shared_io;

// imports
use esp_hal::entry;
use static_cell::StaticCell;
use embassy_executor::Executor;
use embassy_executor::Spawner;
use esp_backtrace as _;
use system::System;

// globals
static EXECUTOR: StaticCell<Executor> = StaticCell::new();


#[entry]
fn main() -> ! {
    // a bit of a hack to get around not being able to use `main` with `#[embassy_executor::main]` on riscv
    // spawn tasks
    let executor = EXECUTOR.init(Executor::new());
    executor.run(|spawner| {
        spawner.spawn(start(spawner)).unwrap();
    });
}


#[embassy_executor::task]
async fn start (spawner : Spawner) {
    // init system
    System::init();

    // spawn tasks
    spawner.spawn(task_manager::init()).unwrap();
    spawner.spawn(task_clock::init()).unwrap();
    spawner.spawn(task_speed::init()).unwrap();

    // loop
    let mut sub_minutes = signals::CLOCK_MINUTES.subscriber().unwrap();
    loop {
        let val = sub_minutes.next_message_pure().await;
        log::info!("{:02}", val);
    }
}