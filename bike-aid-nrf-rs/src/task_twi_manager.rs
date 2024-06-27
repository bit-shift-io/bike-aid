use embassy_nrf::{bind_interrupts, gpio::AnyPin, peripherals::{self, TWISPI0}, twim::{self, Twim}};
use defmt::*;

// twim is two wire interface master
// twis is twire interface slave
static TASK_ID : &str = "TWI";


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
    let mut read_buffer = [0u8; 16];
    let write_buffer = &mut [0x00];

    // Start Scan at Address 1 going up to 127
    info!("{} : Scan TWI", TASK_ID);
    for address in 1..=127 {
        //let mut result: Result<(), twim::Error> = Ok(());
        let mut result: Result<(), twim::Error> = Err(twim::Error::AddressNack);

        let _ = embassy_futures::poll_once(async {
            result = twi.write_read(address, write_buffer, &mut read_buffer).await;
        });

        //info!("Read: {=[u8]:x}", read_buffer);

        // Check and Print Result
        match result {
            Ok(_) => info!("I2C Device Found at Address {}", address as u8),
            Err(_) => {},
        }
    };
    info!("{} : End scan TWI", TASK_ID);
     
    info!("{} : Entering main loop", TASK_ID);
    loop {
    }

}