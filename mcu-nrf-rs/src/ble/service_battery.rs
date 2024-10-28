use nrf_softdevice::ble::gatt_server::builder::ServiceBuilder;
use nrf_softdevice::ble::gatt_server::characteristic::{Attribute, Metadata, Presentation, Properties};
use nrf_softdevice::ble::gatt_server::{CharacteristicHandles, RegisterError};
use nrf_softdevice::ble::{Connection, Uuid};
use nrf_softdevice::{raw, Softdevice};

const BATTERY_SERVICE: Uuid = Uuid::new_16(0x180f);
const BATTERY_LEVEL: Uuid = Uuid::new_16(0x2a19);
//const BATTERY_VOLTAGE: Uuid = Uuid::new_16(0x2B18);
const BATTERY_POWER: Uuid = Uuid::new_16(0x2b05);
//const BATTERY_CURRENT: Uuid = Uuid::new_16(0x2AEE);
//const BATTERY_CAPACITY: Uuid = Uuid::new_16(0x2B06);

// battery service
pub struct BatteryService {
    pub level: CharacteristicHandles,
    //pub voltage: CharacteristicHandles,
    pub power: CharacteristicHandles,
    //pub current: CharacteristicHandles,
    //pub capacity: CharacteristicHandles,
}

impl BatteryService {
    pub fn new(sd: &mut Softdevice) -> Result<Self, RegisterError> {
        let mut service_builder = ServiceBuilder::new(sd, BATTERY_SERVICE)?;

        let characteristic_builder = service_builder.add_characteristic(
            BATTERY_LEVEL, 
            Attribute::new(&[0u8]), 
            Metadata::new(Properties::new().read().notify()).presentation(Presentation {
                format: raw::BLE_GATT_CPF_FORMAT_UINT8 as u8, // unsigned uint 8
                exponent: 0,  /* Value * 10 ^ 0 */
                unit: 0x27AD, /* Percentage */
                name_space: raw::BLE_GATT_CPF_NAMESPACE_BTSIG as u8, // assigned by Bluetooth SIG
                description: raw::BLE_GATT_CPF_NAMESPACE_DESCRIPTION_UNKNOWN as u16, // unknown
            }))?;
        let level = characteristic_builder.build();

        // let characteristic_builder = service_builder.add_characteristic(
        //     BATTERY_VOLTAGE,
        //     Attribute::new(&[0u8]),
        //     Metadata::new(Properties::new().read().notify()).presentation(Presentation {
        //         format: raw::BLE_GATT_CPF_FORMAT_UINT8 as u8, // unsigned uint 8
        //         exponent: 0,  /* Value * 10 ^ 0 */
        //         unit: 0x27AD, /* Percentage */
        //         name_space: raw::BLE_GATT_CPF_NAMESPACE_BTSIG as u8, // assigned by Bluetooth SIG
        //         description: raw::BLE_GATT_CPF_NAMESPACE_DESCRIPTION_UNKNOWN as u16, // unknown
        //     }))?;
        // let voltage_handle = characteristic_builder.build();

        let characteristic_builder = service_builder.add_characteristic(
            BATTERY_POWER,
            Attribute::new(&[0u8, 0u8]),
            Metadata::new(Properties::new().read().notify()),
        )?;
        let power = characteristic_builder.build();

        // let characteristic_builder = service_builder.add_characteristic(
        //     BATTERY_CURRENT,
        //     Attribute::new(&[0u8]),
        //     Metadata::new(Properties::new().read().notify()),
        // )?;
        // let current_handle = characteristic_builder.build();

        // let characteristic_builder = service_builder.add_characteristic(
        //     BATTERY_CAPACITY,
        //     Attribute::new(&[0u8]),
        //     Metadata::new(Properties::new().read().notify()),
        // )?;
        // let capacity_handle = characteristic_builder.build();


        let _service_handle = service_builder.build();

        Ok(BatteryService {
            level,
            //voltage: voltage_handle,
            power,
            //current: current_handle,
            //capacity: capacity_handle,
        })
    }


    pub fn on_write(&self, _connection: &Connection, _handle: u16, _data: &[u8]) {
        // if handle == self.level.cccd_handle {
        //     //info!("battery level notifications: {}", (data[0] & 0x01) != 0);
        // }

        // if handle == self.voltage.cccd_handle {
        //     //info!("battery voltage notifications: {}", (data[0] & 0x01) != 0);
        // }

        // if handle == self.power.cccd_handle {
        //     //info!("battery power notifications: {}", (data[0] & 0x01) != 0);
        // }

        // if handle == self.current.cccd_handle {
        //     //info!("battery current notifications: {}", (data[0] & 0x01) != 0);
        // }

        // if handle == self.capacity.cccd_handle {
        //     //info!("battery capacity notifications: {}", (data[0] & 0x01) != 0);
        // }
    }
}