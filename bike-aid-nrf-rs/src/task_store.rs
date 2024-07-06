use crate::signals;
use defmt::*;
use embassy_time::{Duration, Timer};
use embassy_nrf::nvmc::Nvmc;
use embedded_storage::nor_flash::{NorFlash, ReadNorFlash};

static TASK_ID : &str = "STORE";

#[embassy_executor::task]
pub async fn store (
    mut flash : Nvmc<'static>
) {
    // https://github.com/embassy-rs/embassy/blob/main/examples/nrf52840/src/bin/nvmc.rs

    /*
    const ADDR: u32 = 0x80000;

    info!("Reading...");
    let mut buf = [0u8; 4];
    unwrap!(flash.read(ADDR, &mut buf));
    info!("Read: {=[u8]:x}", buf);

    info!("Erasing...");
    unwrap!(flash.erase(ADDR, ADDR + 4096));

    info!("Reading...");
    let mut buf = [0u8; 4];
    unwrap!(flash.read(ADDR, &mut buf));
    info!("Read: {=[u8]:x}", buf);

    info!("Writing...");
    unwrap!(flash.write(ADDR, &[1, 2, 3, 4]));

    info!("Reading...");
    let mut buf = [0u8; 4];
    unwrap!(flash.read(ADDR, &mut buf));
    info!("Read: {=[u8]:x}", buf);
 */
    // TODO: 
    // - read settings from memory and apply them
    // - subscribe to all signals and store them in the flash
    /*
    info!("{} : Entering main loop",TASK_ID);
    loop {
    }
     */
}