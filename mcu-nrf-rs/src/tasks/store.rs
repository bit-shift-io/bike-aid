use crate::utils::{signals, store};
use defmt::*;
use embassy_nrf::nvmc::Nvmc;
use embedded_storage_async::nor_flash::MultiwriteNorFlash;

const TASK_ID: &str = "STORE";
const FLASH_ADDRESS: u32 = 0x80000;
const BYTE_SIZE: u32 = 8;


#[embassy_executor::task]
pub async fn task(
    flash_controller: Nvmc<'static>
) {
    info!("{}: start", TASK_ID);
    // https://github.com/embassy-rs/embassy/blob/main/examples/nrf52840/src/bin/nvmc.rs
    // https://github.com/tweedegolf/sequential-storage/blob/master/example/src/main.rs

    let mut flash = embassy_embedded_hal::adapter::BlockingAsync::new(flash_controller);

    // TODO: load store
    //read_store(&mut flash).await;

    // erase flash
    // shoudnt need this?
    //unwrap!(flash.erase(address, address + size));

    let mut sub_write = signals::STORE_WRITE.subscriber().unwrap();

    loop {
        if sub_write.next_message_pure().await {
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
    let mut throttle_settings = store::THROTTLE_SETTINGS.lock().await;

    write_bool(flash, &mut offset, &mut throttle_settings.passthrough).await;
    write_i16(flash, &mut offset, &mut throttle_settings.increase_smooth_factor).await;
    write_i16(flash, &mut offset, &mut throttle_settings.decrease_smooth_factor).await;
    write_i16(flash, &mut offset, &mut throttle_settings.no_throttle).await;
    write_i16(flash, &mut offset, &mut throttle_settings.full_throttle).await;
    write_i16(flash, &mut offset, &mut throttle_settings.deadband_min).await;
    write_i16(flash, &mut offset, &mut throttle_settings.deadband_max).await;
    write_i16(flash, &mut offset, &mut throttle_settings.speed_limit).await;
    // == settings end ==
}


async fn read_store<E: defmt::Format>(
    flash: &mut impl MultiwriteNorFlash<Error = E>
) {
    info!("{}: read store", TASK_ID);

    let pub_updated = signals::STORE_UPDATED.publisher().unwrap();
    let mut offset = 0; // address read offset

    // == settings begin ==
    let mut throttle_settings = store::THROTTLE_SETTINGS.lock().await;
    read_bool(flash, &mut offset, &mut throttle_settings.passthrough).await;
    read_i16(flash, &mut offset, &mut throttle_settings.increase_smooth_factor).await;
    read_i16(flash, &mut offset, &mut throttle_settings.decrease_smooth_factor).await;
    read_i16(flash, &mut offset, &mut throttle_settings.no_throttle).await;
    read_i16(flash, &mut offset, &mut throttle_settings.full_throttle).await;
    read_i16(flash, &mut offset, &mut throttle_settings.deadband_min).await;
    read_i16(flash, &mut offset, &mut throttle_settings.deadband_max).await;
    read_i16(flash, &mut offset, &mut throttle_settings.speed_limit).await;

    // == settings end ==

    // notify
    pub_updated.publish_immediate(true);
}


async fn write_bool(flash: &mut impl MultiwriteNorFlash, offset: &mut u32, setting: &mut bool) {
    let mut buf = [0u8; 1];
    if let Ok(_result) = flash.read(FLASH_ADDRESS + (*offset * BYTE_SIZE), &mut buf).await {
        if *setting != buf.iter().any(|&x| x != 0) { // bool
            let _ = flash.write(FLASH_ADDRESS + (*offset * BYTE_SIZE), &mut [*setting as u8]).await;
        }
    }
    *offset += 1; // bool
}


async fn write_i16(flash: &mut impl MultiwriteNorFlash, offset: &mut u32, setting: &mut i16) {
    let mut buf = [0u8; 2];
    if let Ok(_result) = flash.read(FLASH_ADDRESS + (*offset * BYTE_SIZE), &mut buf).await {
        if *setting != i16::from_le_bytes([buf[0], buf[1]]) { // i16
            let _ = flash.write(FLASH_ADDRESS + (*offset * BYTE_SIZE), &setting.to_le_bytes()).await;
        }
    }
    *offset += 2; // i16
}


async fn read_bool(flash: &mut impl MultiwriteNorFlash, offset: &mut u32, setting: &mut bool) {
    let mut buf = [0u8; 1];
    if let Ok(_result) = flash.read(FLASH_ADDRESS + (*offset * BYTE_SIZE), &mut buf).await {
        *setting = buf.iter().any(|&x| x != 0) as bool; // Convert bool to usize
    }
    *offset += 1; // Increment offset for bool
}


async fn read_i16(flash: &mut impl MultiwriteNorFlash, offset: &mut u32, setting: &mut i16) {
    let mut buf = [0u8; 2];
    if let Ok(_result) = flash.read(FLASH_ADDRESS + (*offset * BYTE_SIZE), &mut buf).await {
        let value = i16::from_le_bytes([buf[0], buf[1]]);
        if value != 0 {
            *setting = value; // Convert i16 to usize
        }
    }
    *offset += 2; // Increment offset for i16
}