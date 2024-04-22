use embassy_executor::Spawner;

use super::trip::clock::Clock;
pub mod clock;

pub trait TripTrait {
    fn start(&mut self);
    fn stop(&mut self);
}

pub struct Trip {
    spawner: Spawner
}

impl Trip {
    pub fn new(spawner: Spawner) -> Self {
        Self {
            spawner
        }
    }
    
}

impl TripTrait for Trip {
    fn start(&mut self) {
        log::info!("Trip started");

        // start clock
        let mut clock: Clock = Clock::new(self.spawner);
        clock.start();  
    }

    fn stop(&mut self) {
    }
}