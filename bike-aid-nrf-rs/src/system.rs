
use embassy_nrf::twim::{self, Twim};
use {defmt_rtt as _, panic_probe as _};
use embassy_nrf::{bind_interrupts, peripherals};
use defmt::*;


const ADDRESS: u8 = 0x50;
static TASK_ID : &str = "System";

bind_interrupts!(struct Irqs {
    SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0 => twim::InterruptHandler<peripherals::TWISPI0>;
});

pub struct System {
    //peripherals: &'static Peripherals
}

impl System {
    pub fn init() -> Self {
        log::info!("{} : Entering main loop", TASK_ID);

        let p = embassy_nrf::init(Default::default());
        
        info!("Initializing TWI...");
        let config = twim::Config::default();
        let mut twi = Twim::new(p.TWISPI0, Irqs, p.P0_03, p.P0_04, config);
    
        info!("Reading...");
    
        let mut buf = [0u8; 16];
        unwrap!(twi.blocking_write_read(ADDRESS, &mut [0x00], &mut buf));
    
        info!("Read: {=[u8]:x}", buf);

        Self {
        //    peripherals: &Peripherals::take()
        }
    }

    /*
    pub fn init_i2c() {
        // I2C
        log::info!("I2C : init");

        // Start Scan at Address 1 going up to 127
        for addr in 1..=127 {
            // Scan Address
            let res = i2c0.read(addr as u8, &mut [0]);
    
            // Check and Print Result
            match res {
                Ok(_) => log::info!("I2C Device Found at Address {}", addr as u8),
                Err(_) => {},
            }
        };
    }
     */
}