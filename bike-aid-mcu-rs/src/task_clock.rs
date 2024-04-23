use crate::{signals, task_manager::TaskTrait};
use embassy_executor::{Executor, Spawner};
use embassy_time::{Duration, Timer};
use static_cell::StaticCell;

static TASK_ID : &str = "CLOCK";

#[embassy_executor::task]
pub async fn clock () {
    let pub1 = signals::TEST_CHANNEL.publisher().unwrap();

    log::info!("{} : Entering main loop",TASK_ID);
    loop {
        pub1.publish_immediate(1);
        Timer::after(Duration::from_millis(1100)).await;
    }
}

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


}


impl TaskTrait for Clock  {
    fn start(&mut self) {
        self.active = true;
        log::info!("starting clock");
    }

    fn stop(&mut self) {
        self.active = false;
    }
}

#[embassy_executor::task]
async fn run() {
    let mut count = 0;
    loop {
        log::info!("Clock Count: {}", count);
        count += 1;
        Timer::after(Duration::from_millis(60_000)).await;
    }
}