use crate::trip::TripTrait;
use embassy_executor::{Executor, Spawner};
use embassy_time::{Duration, Timer};
use static_cell::StaticCell;


pub struct Clock {
    pub active: bool,
    pub start_time: u32,
    interval: u8,
    last_interval: u8,
    executor:  StaticCell<Executor>, // low priority task
    spawner: Spawner
}


impl Clock {
    pub fn new(spawner: Spawner) -> Self {
        Self {
            active: false,
            start_time: 0,
            interval: 0,
            last_interval: 0,
            executor: StaticCell::new(),
            spawner
        }
    }

    #[embassy_executor::task]
    async fn run(&mut self) {
        let mut count = 0;
        loop {
            log::info!("Clock Count: {}", count);
            count += 1;
            Timer::after(Duration::from_millis(60_000)).await;
        }
    }
}


impl TripTrait for Clock  {
    fn start(&mut self) {
        self.active = true;
        log::info!("starting clock");
        self.spawner.spawn(self.run()).ok();
    }

    fn stop(&mut self) {
        self.active = false;
    }
}