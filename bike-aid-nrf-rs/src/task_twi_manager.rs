use embassy_nrf::{bind_interrupts, gpio::AnyPin, peripherals::{self, TWISPI0}, twim::{self, Twim}};
use defmt::*;

// twim is two wire interface master
// twis is twire interface slave
static TASK_ID : &str = "TWI";


#[embassy_executor::task]
pub async fn twi_manager (
    port_twi : TWISPI0,
    pin_sda : AnyPin,
    pin_scl : AnyPin
) {
    info!("{} : Initializing", TASK_ID);

    // bind interrupts
    bind_interrupts!(struct Irqs {
        SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0 => twim::InterruptHandler<peripherals::TWISPI0>;
    });

    let config = twim::Config::default();
    let mut twi = Twim::new(port_twi, Irqs, pin_sda, pin_scl, config);
    let mut read_buffer = [0u8; 16];
    let write_buffer = &mut [0x00];

    //scan_devices
    for address in 0..128 {
        match twi.write(address, &[]).await {
            Ok(_) => {
                info!("Device found at address: 0x{:X}", address);
            }
            Err(_) => continue,
        }
    }

     /*
    info!("{} : Entering main loop", TASK_ID);
    loop {
    }
     */

}