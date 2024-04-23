use crate::{signals, task_manager::TaskTrait};
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use static_cell::StaticCell;

static TASK_ID : &str = "SPEED";

#[embassy_executor::task]
pub async fn speed () {
    let pub1 = signals::TEST_CHANNEL.publisher().unwrap();

    log::info!("{} : Entering main loop",TASK_ID);
    loop {
        pub1.publish_immediate(2);
        Timer::after(Duration::from_millis(1000)).await;
    }
}

pub struct Speed {
    pub active: bool,
    pub start_time: u32,
    interval: u8,
    last_interval: u8
}


impl Speed {
    pub fn new() -> Self {
        Self {
            active: false,
            start_time: 0,
            interval: 0,
            last_interval: 0,
        }
    }
    
}

impl TaskTrait for Speed  {
    fn start(&mut self) {
        self.active = true;
        log::info!("starting Speed");
    }

    fn stop(&mut self) {
        self.active = false;
    }
}