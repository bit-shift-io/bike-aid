#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

// imports
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp32c3_hal::{clock::ClockControl, embassy, peripherals::Peripherals, prelude::*};
use esp_backtrace as _;
use esp_println::logger::init_logger;

// modules
mod trip;
use trip::{Trip, TripTrait};


#[embassy_executor::task]
async fn one_second_task() {
    let mut count = 0;
    loop {
        log::info!("Spawn Task Count: {}", count);
        //esp_println::println!("Spawn Task Count: {}", count);
        count += 1;
        Timer::after(Duration::from_millis(1_000)).await;
    }
}



#[main]
async fn main(spawner: Spawner) {
    // init esp32 stuff
    init_logger(log::LevelFilter::Info); // esp32 logger
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    // init embassy
    embassy::init(
        &clocks,
        esp32c3_hal::timer::TimerGroup::new(peripherals.TIMG0, &clocks).timer0,
    );

    // init trip
    let mut trip: Trip = Trip::new(spawner);
    trip.start();

    
    // loop
    let mut count = 0;
    loop {
        log::info!("Main Task Count: {}", count);
        count += 1;
        Timer::after(Duration::from_millis(5_000)).await;
    }
}
