use embassy_nrf::{bind_interrupts, gpio::AnyPin, peripherals::{self, TWISPI0}, twim::{self, Twim}};
use defmt::*;

// twim is two wire interface master
// twis is twire interface slave
static TASK_ID : &str = "TWI MANAGER";

const ADDRESS: u8 = 0x50;


#[embassy_executor::task]
pub async fn twi_manager (
    twi_io : TWISPI0,
    pin_sda : AnyPin,
    pin_scl : AnyPin
) {
    info!("{} : Initializing", TASK_ID);

    // bind interrupts
    bind_interrupts!(struct Irqs {
        SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0 => twim::InterruptHandler<peripherals::TWISPI0>;
    });

    let config = twim::Config::default();
    let mut twi = Twim::new(twi_io, Irqs, pin_sda, pin_scl, config);
    let mut buf = [0u8; 16];
 
    // Start Scan at Address 1 going up to 127
    info!("{} : Scan TWI", TASK_ID);
    for addr in 1..=127 {
        // Scan Address
        //let res = i2c0.read(addr as u8, &mut [0]); // old esp32
        unwrap!(twi.blocking_write_read(addr, &mut [0x00], &mut buf));
        info!("Read: {=[u8]:x}", buf);

        /*
        // Check and Print Result
        match res {
            Ok(_) => info!("I2C Device Found at Address {}", addr as u8),
            Err(_) => {},
        }
         */
    };
    info!("{} : End scan TWI", TASK_ID);


    info!("{} : Entering main loop",TASK_ID);
    loop {
    }

}