#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

//use riscv_rt::entry;

// imports
use embassy_executor::Spawner;
use esp32c3_hal::{clock::ClockControl, embassy, peripherals::Peripherals, prelude::*, IO};
use esp_backtrace as _;
use esp_println::logger::init_logger;

// modules
mod signals;
mod task_manager;
mod task_clock;
mod task_speed;
use task_speed::speed;
use task_clock::clock;
use task_manager::TaskManager;



#[main]
async fn main(spawner: Spawner) {
    // init esp32 stuff
    init_logger(log::LevelFilter::Info); // esp32 logger
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let pins = io.pins;

    // init embassy
    embassy::init(
        &clocks,
        esp32c3_hal::timer::TimerGroup::new(peripherals.TIMG0, &clocks).timer0,
    );

    // init task manager
    let mut task_manager: TaskManager = TaskManager::new(spawner);

    // spawn tasks
    spawner.spawn(clock()).unwrap();
    spawner.spawn(speed()).unwrap();

    
    // loop
    let mut sub_minutes = signals::CHANNEL_CLOCK_MINUTES.subscriber().unwrap();
    loop {
        let val = sub_minutes.next_message_pure().await;
        log::info!("{:02}", val);
    }
}
