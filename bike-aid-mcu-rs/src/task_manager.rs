use embassy_executor::Spawner;

use crate::task_clock::Clock;

pub trait TaskTrait {
    fn start(&mut self);
    fn stop(&mut self);
}

pub struct TaskManager {
    spawner: Spawner
}

impl TaskManager {
    pub fn new(spawner: Spawner) -> Self {
        Self {
            spawner
        }
    }
    
}

impl TaskTrait for TaskManager {
    fn start(&mut self) {
        log::info!("Trip started");

        // start clock
        let mut clock: Clock = Clock::new(self.spawner);
        clock.start();  
    }

    fn stop(&mut self) {
    }
}