use defmt::*;
use nrf_softdevice::ble::gatt_server::builder::ServiceBuilder;
use nrf_softdevice::ble::gatt_server::characteristic::{Attribute, Metadata, Properties};
use nrf_softdevice::ble::gatt_server::{CharacteristicHandles, RegisterError};
use nrf_softdevice::ble::{Connection, Uuid};
use nrf_softdevice::Softdevice;

// fast pair locator tags
// https://developers.google.com/nearby/fast-pair/specifications/service/gatt
// https://developers.google.com/nearby/fast-pair/specifications/service/provider#provider_advertising_signal
// https://developers.google.com/nearby/fast-pair/specifications/devicefeaturerequirement/devicefeaturerequirement_locatortags
// https://developers.google.com/nearby/fast-pair/specifications/extensions/fmdn
// https://github.com/nrfconnect/sdk-nrf/blob/fa50704e0282a7ea72aad50ca48619d68c607201/include/bluetooth/services/fast_pair/uuid.h
// https://developer.nordicsemi.com/nRF_Connect_SDK/doc-legacy/2.7.0/nrf/samples/bluetooth/fast_pair/locator_tag/README.html
// NCS locator tag:
// Device Name: NCS locator tag
// Model ID: 0x4A436B
// Anti-Spoofing Private Key (base64, uncompressed): rie10A7ONqwd77VmkxGsblPUbMt384qjDgcEJ/ctT9Y=
// Device Type: Locator Tag
// Notification Type: Fast Pair
// Data-Only connection: true
// No Personalized Name: true
// Find My Device: true

const UUID_FAST_PAIR_SERIVCE: u16 = 0xFE2C;

// FE2C1233-8366-4814-8EB0-01DE32100BEA
const UUID_MODEL_ID: [u8; 16] = [0xEA, 0x0B, 0x10, 0x32, 0xDE, 0x01, 0xB0, 0x8E, 0x14, 0x48, 0x66, 0x83, 0x33, 0x12, 0x2C, 0xFE];

// FE2C1238-8366-4814-8EB0-01DE32100BEA
const UUID_BEACON_ACTIONS: [u8; 16] = [0xEA, 0x0B, 0x10, 0x32, 0xDE, 0x01, 0xB0, 0x8E, 0x14, 0x48, 0x66, 0x83, 0x38, 0x12, 0x2C, 0xFE];

// FE2C1234-8366-4814-8EB0-01DE32100BEA
const UUID_KEY_BASED_PAIRING: [u8; 16] = [0xEA, 0x0B, 0x10, 0x32, 0xDE, 0x01, 0xB0, 0x8E, 0x14, 0x48, 0x66, 0x83, 0x34, 0x12, 0x2C, 0xFE];

// FE2C1235-8366-4814-8EB0-01DE32100BEA
const UUID_PASSKEY: [u8; 16] = [0xEA, 0x0B, 0x10, 0x32, 0xDE, 0x01, 0xB0, 0x8E, 0x14, 0x48, 0x66, 0x83, 0x35, 0x12, 0x2C, 0xFE];

// FE2C1236-8366-4814-8EB0-01DE32100BEA
const UUID_ACCOUNT_KEY: [u8; 16] = [0xEA, 0x0B, 0x10, 0x32, 0xDE, 0x01, 0xB0, 0x8E, 0x14, 0x48, 0x66, 0x83, 0x36, 0x12, 0x2C, 0xFE];

// FE2C1237-8366-4814-8EB0-01DE32100BEA
const UUID_ADDITIONAL_DATA: [u8; 16] = [0xEA, 0x0B, 0x10, 0x32, 0xDE, 0x01, 0xB0, 0x8E, 0x14, 0x48, 0x66, 0x83, 0x37, 0x12, 0x2C, 0xFE];

const MODEL_ID: [u8; 3] = [0x4A, 0x43, 0x6B];


pub struct FastPairService {
    pub model_id: CharacteristicHandles,
    pub beacon_actions: CharacteristicHandles,
    pub key_based_pairing: CharacteristicHandles,
    pub passkey: CharacteristicHandles,
    pub account_key: CharacteristicHandles,
    pub additional_data: CharacteristicHandles,
}


impl FastPairService {
    pub fn new(sd: &mut Softdevice) -> Result<Self, RegisterError> {
        let mut service_builder = ServiceBuilder::new(sd, Uuid::new_16(UUID_FAST_PAIR_SERIVCE))?;

        let characteristic_builder = service_builder.add_characteristic(
            Uuid::new_128(&UUID_MODEL_ID),
            Attribute::new(&MODEL_ID),
            Metadata::new(Properties::new().read()),
        )?;
        let model_id = characteristic_builder.build();

        let characteristic_builder = service_builder.add_characteristic(
            Uuid::new_128(&UUID_BEACON_ACTIONS),
            Attribute::new(&[0u8]),
            Metadata::new(Properties::new().write()), // .notify()
        )?;
        let beacon_actions = characteristic_builder.build();

        let characteristic_builder = service_builder.add_characteristic(
            Uuid::new_128(&UUID_KEY_BASED_PAIRING),
            Attribute::new(&[0u8]),
            Metadata::new(Properties::new().write().notify()),
        )?;
        let key_based_pairing = characteristic_builder.build();

        let characteristic_builder = service_builder.add_characteristic(
            Uuid::new_128(&UUID_PASSKEY),
            Attribute::new(&[0u8]),
            Metadata::new(Properties::new().write().notify()),
        )?;
        let passkey = characteristic_builder.build();

        let characteristic_builder = service_builder.add_characteristic(
            Uuid::new_128(&UUID_ACCOUNT_KEY),
            Attribute::new(&[0u8]),
            Metadata::new(Properties::new().write()),
        )?;
        let account_key = characteristic_builder.build();

        let characteristic_builder = service_builder.add_characteristic(
            Uuid::new_128(&UUID_ADDITIONAL_DATA),
            Attribute::new(&[0u8]),
            Metadata::new(Properties::new().write().notify()),
        )?;
        let additional_data = characteristic_builder.build();

        let _service_handle = service_builder.build();
        
        Ok(FastPairService {
            model_id,
            beacon_actions,
            key_based_pairing,
            passkey,
            account_key,
            additional_data
        })
    }


    pub fn on_write(&self, _connection: &Connection, handle: u16, data: &[u8]) {
        if handle == self.beacon_actions.cccd_handle {
            info!("beacon_actions on_write");
        }

        if handle == self.beacon_actions.sccd_handle {
            info!("beacon_actions on_write");
        }

        if handle == self.beacon_actions.value_handle {
            info!("beacon_actions on_write");
        }

        if handle == self.key_based_pairing.cccd_handle {
            info!("key_based_pairing on_write");
        }

        if handle == self.key_based_pairing.value_handle {
            info!("key_based_pairing on_write");
        }

        if handle == self.key_based_pairing.sccd_handle {
            info!("key_based_pairing on_write");
        }
    }
}