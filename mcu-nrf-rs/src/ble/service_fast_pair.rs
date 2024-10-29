use defmt::*;
use nrf_softdevice::ble::gatt_server::builder::ServiceBuilder;
use nrf_softdevice::ble::gatt_server::characteristic::{Attribute, Metadata, Properties};
use nrf_softdevice::ble::gatt_server::{CharacteristicHandles, RegisterError};
use nrf_softdevice::ble::{Connection, Uuid};
use nrf_softdevice::Softdevice;
use crate::utils::{functions, globals};
use crate::utils::signals;

// fast pair locator tags
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


// little endian, so reverse order for bytes!
// uart service: 6E400001-B5A3-F393-E0A9-E50E24DCCA9E
// model id: FE2C1233-8366-4814-8EB0-01DE32100BEA

const FAST_PAIR_SERIVCE: u16 = 0xFE2C;
const MODEL_ID: [u8; 16] = [
    0xFE, 0x2C, 0x12, 0x33,
    0x83, 0x66,
    0x48, 0x14,
    0x8E, 0xB0,
    0x01, 0xDE, 0x32, 0x10, 0x0B, 0xEA,
];
const BEACON_ACTIONS: [u8; 16] = [
    0xFE, 0x2C, 0x12, 0x38,
    0x83, 0x66,
    0x48, 0x14,
    0x8E, 0xB0,
    0x01, 0xDE, 0x32, 0x10, 0x0B, 0xEA,
];
/*

const KEY_BASED_PAIRING_ID: [u8; 16] = [
    0x34, 0x12, 0x2C, 0xFE, // Time-low (little-endian)
    0x66, 0x83,             // Time-mid (big-endian)
    0x14, 0x48,             // Time-high (big-endian)
    0x8E, 0xB0,             // Clock-seq (big-endian)
    0xDE, 0x01, 0x00, 0xBE, // Node (big-endian)
    0xA0, 0x32, 0x21, 0xDE  // Node continued (big-endian)
];
const PASSKEY_ID: [u8; 16] = [
    0x35, 0x12, 0x2C, 0xFE, // Time-low (little-endian)
    0x66, 0x83,             // Time-mid (big-endian)
    0x14, 0x48,             // Time-high (big-endian)
    0x8E, 0xB0,             // Clock-seq (big-endian)
    0xDE, 0x01, 0x00, 0xBE, // Node (big-endian)
    0xA0, 0x32, 0x21, 0xDE  // Node continued (big-endian)
];
const ACCOUNT_KEY_ID: [u8; 16] = [
    0x36, 0x12, 0x2C, 0xFE, // Time-low (little-endian)
    0x66, 0x83,             // Time-mid (big-endian)
    0x14, 0x48,             // Time-high (big-endian)
    0x8E, 0xB0,             // Clock-seq (big-endian)
    0xDE, 0x01, 0x00, 0xBE, // Node (big-endian)
    0xA0, 0x32, 0x21, 0xDE  // Node continued (big-endian)
];
const ADDITIONAL_DATA_ID: [u8; 16] = [
    0x37, 0x12, 0x2C, 0xFE, // Time-low (little-endian)
    0x66, 0x83,             // Time-mid (big-endian)
    0x14, 0x48,             // Time-high (big-endian)
    0x8E, 0xB0,             // Clock-seq (big-endian)
    0xDE, 0x01, 0x00, 0xBE, // Node (big-endian)
    0xA0, 0x32, 0x21, 0xDE  // Node continued (big-endian)
];
const BEACON_ACTIONS_ID: [u8; 16] = [
    0x38, 0x12, 0x2C, 0xFE, // Time-low (little-endian)
    0x66, 0x83,             // Time-mid (big-endian)
    0x14, 0x48,             // Time-high (big-endian)
    0x8E, 0xB0,             // Clock-seq (big-endian)
    0xDE, 0x01, 0x00, 0xBE, // Node (big-endian)
    0xA0, 0x32, 0x21, 0xDE  // Node continued (big-endian)
];
 */

pub struct FastPairService {
    pub model_id: CharacteristicHandles,
    pub beacon_actions: CharacteristicHandles,
}


impl FastPairService {
    pub fn new(sd: &mut Softdevice) -> Result<Self, RegisterError> {
        let mut service_builder = ServiceBuilder::new(sd, Uuid::new_16(FAST_PAIR_SERIVCE))?;

        let characteristic_builder = service_builder.add_characteristic(
            Uuid::new_128(&MODEL_ID),
            Attribute::new(&[0u8]),
            Metadata::new(Properties::new().write()), // .notify()
        )?;
        let model_id = characteristic_builder.build();

        let characteristic_builder = service_builder.add_characteristic(
            Uuid::new_128(&BEACON_ACTIONS),
            Attribute::new(&[0u8]),
            Metadata::new(Properties::new().write()), // .notify()
        )?;
        let beacon_actions = characteristic_builder.build();

        let _service_handle = service_builder.build();
        
        Ok(FastPairService {
            model_id,
            beacon_actions,
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
    }
}