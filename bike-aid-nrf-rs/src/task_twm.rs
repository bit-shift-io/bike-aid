use crate::{signals, system::System};
use embassy_nrf::twim::{self, Twim};
use embassy_time::{Duration, Timer};
use defmt::*;

static TASK_ID : &str = "TW MANAGER";

const ADDRESS: u8 = 0x50;

#[embassy_executor::task]
pub async fn init () {
    info!("Initializing TWI...");
    let config = twim::Config::default();
    let mut twi = Twim::new(System::peripherals.TWISPI0, Irqs, System::peripherals.P0_03, System::peripherals.P0_04, config);

    info!("Reading...");

    //let mut buf = [0u8; 16];
    //unwrap!(twi.blocking_write_read(ADDRESS, &mut [0x00], &mut buf));

    //info!("Read: {=[u8]:x}", buf);

    /*
    
    // Start Scan at Address 1 going up to 127
    for addr in 1..=127 {
        // Scan Address
        let res = i2c0.read(addr as u8, &mut [0]);

        // Check and Print Result
        match res {
            Ok(_) => info!("I2C Device Found at Address {}", addr as u8),
            Err(_) => {},
        }
    };

    */

    info!("{} : Entering main loop",TASK_ID);
    loop {
    }

}