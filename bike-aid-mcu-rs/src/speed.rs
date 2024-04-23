use crate::trip::TripTrait;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use static_cell::StaticCell;
use embassy_executor::{Executor, InterruptExecutor};

pub struct Speed {
    pub active: bool,
    pub start_time: u32,
    interval: u8,
    last_interval: u8,
    executor_high: InterruptExecutor // high priority task
}


impl Speed {
    pub fn new() -> Self {
        Self {
            active: false,
            start_time: 0,
            interval: 0,
            last_interval: 0,
            executor_high: InterruptExecutor::new()
        }
    }

    #[embassy_executor::task]
    async fn run(&mut self) {
        let mut count = 0;
        loop {
            log::info!("Speed Count: {}", count);
            count += 1;
            Timer::after(Duration::from_millis(1_000)).await;
        }
    }

    #[interrupt]
    unsafe fn SWI1_EGU1() {
        executor_high.on_interrupt()
    }
    
}

impl TripTrait for Speed  {
    fn start(&mut self) {
        self.active = true;
        log::info!("starting Speed");

        // High-priority executor: SWI1_EGU1, priority level 6
        interrupt::SWI1_EGU1.set_priority(Priority::P6);
        let spawner = executor_high.start(interrupt::SWI1_EGU1);
        unwrap!(spawner.spawn(run_high()));
    }

    fn stop(&mut self) {
        self.active = false;
    }
}