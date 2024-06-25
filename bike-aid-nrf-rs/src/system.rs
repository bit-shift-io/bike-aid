
use embassy_nrf::twim::{self, Twim};
use {defmt_rtt as _, panic_probe as _};
use embassy_nrf::{bind_interrupts, peripherals};
use embassy_nrf::Peripherals;
use defmt::*;

static TASK_ID : &str = "System";


pub struct System {
    pub peripherals: Peripherals
}

impl System {
    pub fn init() -> Self {
        info!("{} : init", TASK_ID);

        let p = embassy_nrf::init(Default::default());
        
        Self {
            peripherals: p
        }
    }


}