use crate::{signals, store};
use defmt::*;
use embassy_nrf::nvmc::Nvmc;
use embedded_storage_async::nor_flash::MultiwriteNorFlash;

const TASK_ID: &str = "STORE";
const FLASH_ADDRESS: u32 = 0x80000;
const BYTE_SIZE: u32 = 8;

#[embassy_executor::task]
pub async fn store (
    flash_controller: Nvmc<'static>
) {
    info!("{}: start", TASK_ID);
    // https://github.com/embassy-rs/embassy/blob/main/examples/nrf52840/src/bin/nvmc.rs
    // https://github.com/tweedegolf/sequential-storage/blob/master/example/src/main.rs

    let mut flash = embassy_embedded_hal::adapter::BlockingAsync::new(flash_controller);

    // load store
    load_store(&mut flash).await;

    // erase flash
    // shoudnt need this?
    //unwrap!(flash.erase(address, address + size));

    let mut sub_write = signals::STORE_WRITE.subscriber().unwrap();

    info!("{} : Entering main loop",TASK_ID);
    loop {
        let val = sub_write.next_message_pure().await;
        if val {
            write_store(&mut flash).await;
        }
    }
}

async fn write_store<E: defmt::Format>(
    flash: &mut impl MultiwriteNorFlash<Error = E>
) {
    info!("{}: write", TASK_ID);

    // read, compare and write on change
    let mut offset = 0; // address read offset

    // == settings begin ==
    let throttle_settings = store::THROTTLE_SETTINGS.lock().await;

    let mut buf = [0u8; 1];
    if let Ok(_result) = flash.read(FLASH_ADDRESS + (offset * BYTE_SIZE), &mut buf).await {
        if throttle_settings.passthrough != buf.iter().any(|&x| x != 0) { // bool
            let _ = flash.write(FLASH_ADDRESS + (offset * BYTE_SIZE), &[throttle_settings.passthrough as u8]).await;
        }
    }
    offset += 1; // bool

    let mut buf = [0u8; 2];
    if let Ok(_result) = flash.read(FLASH_ADDRESS + (offset * BYTE_SIZE), &mut buf).await {
        if throttle_settings.increase_smooth_factor != i16::from_le_bytes([buf[0], buf[1]]) { // i16
            let _ = flash.write(FLASH_ADDRESS + (offset * BYTE_SIZE), &throttle_settings.increase_smooth_factor.to_le_bytes()).await;
        }
    }
    offset += 2; // i16

    let mut buf = [0u8; 2];
    if let Ok(_result) = flash.read(FLASH_ADDRESS + (offset * BYTE_SIZE), &mut buf).await {
        if throttle_settings.decrease_smooth_factor != i16::from_le_bytes([buf[0], buf[1]]) { // i16
            let _ = flash.write(FLASH_ADDRESS + (offset * BYTE_SIZE), &throttle_settings.decrease_smooth_factor.to_le_bytes()).await;
        }
    }
    offset += 2; // i16

    let mut buf = [0u8; 2];
    if let Ok(_result) = flash.read(FLASH_ADDRESS + (offset * BYTE_SIZE), &mut buf).await {
        if throttle_settings.limit_min != i16::from_le_bytes([buf[0], buf[1]]) { // i16
            let _ = flash.write(FLASH_ADDRESS + (offset * BYTE_SIZE), &throttle_settings.limit_min.to_le_bytes()).await;
        }
    }
    offset += 2; // i16

    let mut buf = [0u8; 2];
    if let Ok(_result) = flash.read(FLASH_ADDRESS + (offset * BYTE_SIZE), &mut buf).await {
        if throttle_settings.limit_max != i16::from_le_bytes([buf[0], buf[1]]) { // i16
            let _ = flash.write(FLASH_ADDRESS + (offset * BYTE_SIZE), &throttle_settings.limit_max.to_le_bytes()).await;
        }
    }
    offset += 2; // i16

    let mut buf = [0u8; 2];
    if let Ok(_result) = flash.read(FLASH_ADDRESS + (offset * BYTE_SIZE), &mut buf).await {
        if throttle_settings.deadband_in_min != i16::from_le_bytes([buf[0], buf[1]]) { // i16
            let _ = flash.write(FLASH_ADDRESS + (offset * BYTE_SIZE), &throttle_settings.deadband_in_min.to_le_bytes()).await;
        }
    }
    offset += 2; // i16

    let mut buf = [0u8; 2];
    if let Ok(_result) = flash.read(FLASH_ADDRESS + (offset * BYTE_SIZE), &mut buf).await {
        if throttle_settings.deadband_in_max != i16::from_le_bytes([buf[0], buf[1]]) { // i16
            let _ = flash.write(FLASH_ADDRESS + (offset * BYTE_SIZE), &throttle_settings.deadband_in_max.to_le_bytes()).await;
        }
    }
    offset += 2; // i16

    let mut buf = [0u8; 2];
    if let Ok(_result) = flash.read(FLASH_ADDRESS + (offset * BYTE_SIZE), &mut buf).await {
        if throttle_settings.deadband_out_min != i16::from_le_bytes([buf[0], buf[1]]) { // i16
            let _ = flash.write(FLASH_ADDRESS + (offset * BYTE_SIZE), &throttle_settings.deadband_out_min.to_le_bytes()).await;
        }
    }
    offset += 2; // i16

    let mut buf = [0u8; 2];
    if let Ok(_result) = flash.read(FLASH_ADDRESS + (offset * BYTE_SIZE), &mut buf).await {
        if throttle_settings.deadband_out_max != i16::from_le_bytes([buf[0], buf[1]]) { // i16
            let _ = flash.write(FLASH_ADDRESS + (offset * BYTE_SIZE), &throttle_settings.deadband_out_max.to_le_bytes()).await;
        }
    }
    offset += 2; // i16

    // == settings end ==


}


async fn load_store<E: defmt::Format>(
    flash: &mut impl MultiwriteNorFlash<Error = E>
) {
    info!("Loading store...");

    let pub_updated = signals::STORE_UPDATED.immediate_publisher();
    let mut offset = 0; // address read offset

    // == settings begin ==
    let mut throttle_settings = store::THROTTLE_SETTINGS.lock().await;

    let mut buf = [0u8; 1];
    if let Ok(_result) = flash.read(FLASH_ADDRESS + (offset * BYTE_SIZE), &mut buf).await {
        throttle_settings.passthrough = buf.iter().any(|&x| x != 0); // bool
    }
    offset += 1; // bool

    let mut buf = [0u8; 2];
    if let Ok(_result) = flash.read(FLASH_ADDRESS + (offset * BYTE_SIZE), &mut buf).await {
        throttle_settings.increase_smooth_factor = i16::from_le_bytes([buf[0], buf[1]]); // i16
    }
    offset += 2; // i16

    let mut buf = [0u8; 2];
    if let Ok(_result) = flash.read(FLASH_ADDRESS + (offset * BYTE_SIZE), &mut buf).await {
        throttle_settings.decrease_smooth_factor = i16::from_le_bytes([buf[0], buf[1]]); // i16
    }
    offset += 2; // i16

    let mut buf = [0u8; 2];
    if let Ok(_result) = flash.read(FLASH_ADDRESS + (offset * BYTE_SIZE), &mut buf).await {
        throttle_settings.limit_min = i16::from_le_bytes([buf[0], buf[1]]); // i16
    }
    offset += 2; // i16
    
    let mut buf = [0u8; 2];
    if let Ok(_result) = flash.read(FLASH_ADDRESS + (offset * BYTE_SIZE), &mut buf).await {
        throttle_settings.limit_max = i16::from_le_bytes([buf[0], buf[1]]); // i16
    }
    offset += 2; // i16

    let mut buf = [0u8; 2];
    if let Ok(_result) = flash.read(FLASH_ADDRESS + (offset * BYTE_SIZE), &mut buf).await {
        throttle_settings.deadband_in_min = i16::from_le_bytes([buf[0], buf[1]]); // i16
    }
    offset += 2; // i16

    let mut buf = [0u8; 2];
    if let Ok(_result) = flash.read(FLASH_ADDRESS + (offset * BYTE_SIZE), &mut buf).await {
        throttle_settings.deadband_in_max = i16::from_le_bytes([buf[0], buf[1]]); // i16
    }
    offset += 2; // i16

    let mut buf = [0u8; 2];
    if let Ok(_result) = flash.read(FLASH_ADDRESS + (offset * BYTE_SIZE), &mut buf).await {
        throttle_settings.deadband_out_min = i16::from_le_bytes([buf[0], buf[1]]); // i16
    }
    offset += 2; // i16

    let mut buf = [0u8; 2];
    if let Ok(_result) = flash.read(FLASH_ADDRESS + (offset * BYTE_SIZE), &mut buf).await {
        throttle_settings.deadband_out_max = i16::from_le_bytes([buf[0], buf[1]]); // i16
    }
    offset += 2; // i16

    // == settings end ==



    // notify
    pub_updated.publish_immediate(true);
}