#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

//use riscv_rt::entry;


// imports
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp32c3_hal::{clock::ClockControl, embassy, peripherals::Peripherals, prelude::*, IO};
use esp_backtrace as _;
use esp_println::logger::init_logger;

// modules
mod trip;
mod signals;

// task modules
mod task_commander;
mod task_blinker;


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

    use crate::task_commander::commander;
    spawner.must_spawn(commander(
        signals::BLINKER_MODE.publisher().unwrap(),
    ));

    // blinker crate
    use crate::task_blinker::blinker;
    spawner.must_spawn(blinker(
        signals::BLINKER_MODE.subscriber().unwrap(),
        pins.gpio8.into_push_pull_output().degrade()
    ));

    // Ensure all signal subscribers are used
    assert!(signals::BLINKER_MODE.subscriber().is_err());

    /*
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
     */
}
