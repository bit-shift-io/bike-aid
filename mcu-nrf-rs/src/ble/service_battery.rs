use super::server::{self, *};
use crate::utils::signals;
use defmt::{*};
use nrf_softdevice::ble::gatt_server::builder::ServiceBuilder;
use nrf_softdevice::ble::gatt_server::characteristic::{Attribute, Metadata, Presentation, Properties};
use nrf_softdevice::ble::gatt_server::{CharacteristicHandles, RegisterError};
use nrf_softdevice::ble::{Connection, Uuid};
use nrf_softdevice::{raw, Softdevice};
use embassy_futures::join;

const BATTERY_SERVICE: Uuid = Uuid::new_16(0x180f);
const BATTERY_LEVEL: Uuid = Uuid::new_16(0x2a19);
const BATTERY_VOLTAGE: Uuid = Uuid::new_16(0x2B18);
const BATTERY_POWER: Uuid = Uuid::new_16(0x2B05);
const BATTERY_CURRENT: Uuid = Uuid::new_16(0x2AEE);
const BATTERY_CAPACITY: Uuid = Uuid::new_16(0x2B06);

// battery service
pub struct BatteryService {
    level: CharacteristicHandles,
    voltage: CharacteristicHandles,
    power: CharacteristicHandles,
    current: CharacteristicHandles,
    capacity: CharacteristicHandles,
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
        let level_handle = characteristic_builder.build();

        let characteristic_builder = service_builder.add_characteristic(
            BATTERY_VOLTAGE,
            Attribute::new(&[0u8]),
            Metadata::new(Properties::new().read().notify()).presentation(Presentation {
                format: raw::BLE_GATT_CPF_FORMAT_UINT8 as u8, // unsigned uint 8
                exponent: 0,  /* Value * 10 ^ 0 */
                unit: 0x27AD, /* Percentage */
                name_space: raw::BLE_GATT_CPF_NAMESPACE_BTSIG as u8, // assigned by Bluetooth SIG
                description: raw::BLE_GATT_CPF_NAMESPACE_DESCRIPTION_UNKNOWN as u16, // unknown
            }))?;
        let voltage_handle = characteristic_builder.build();

        let characteristic_builder = service_builder.add_characteristic(
            BATTERY_POWER,
            Attribute::new(&[0u8]),
            Metadata::new(Properties::new().read().notify()),
        )?;
        let power_handle = characteristic_builder.build();

        let characteristic_builder = service_builder.add_characteristic(
            BATTERY_CURRENT,
            Attribute::new(&[0u8]),
            Metadata::new(Properties::new().read().notify()),
        )?;
        let current_handle = characteristic_builder.build();

        let characteristic_builder = service_builder.add_characteristic(
            BATTERY_CAPACITY,
            Attribute::new(&[0u8]),
            Metadata::new(Properties::new().read().notify()),
        )?;
        let capacity_handle = characteristic_builder.build();


        let _service_handle = service_builder.build();

        Ok(BatteryService {
            level: level_handle,
            voltage: voltage_handle,
            power: power_handle,
            current: current_handle,
            capacity: capacity_handle,
        })
    }
/*
    pub fn battery_level_get(&self, sd: &Softdevice) -> Result<u8, gatt_server::GetValueError> {
        let buf = &mut [0u8];
        gatt_server::get_value(sd, self.level.value_handle, buf)?;
        Ok(buf[0])
    }

    pub fn battery_level_set(&self, sd: &Softdevice, val: u8) -> Result<(), gatt_server::SetValueError> {
        gatt_server::set_value(sd, self.level.value_handle, &[val])
    }
    pub fn battery_level_notify(&self, conn: &Connection, val: u8) -> Result<(), gatt_server::NotifyValueError> {
        gatt_server::notify_value(conn, self.level.value_handle, &[val])
    }
 */
    pub fn on_write(&self, _connection: &Connection, handle: u16, data: &[u8]) {
        if data.is_empty() {
            return;
        }

        if handle == self.level.cccd_handle {
            //info!("battery level notifications: {}", (data[0] & 0x01) != 0);
        }

        if handle == self.voltage.cccd_handle {
            //info!("battery voltage notifications: {}", (data[0] & 0x01) != 0);
        }

        if handle == self.power.cccd_handle {
            //info!("battery power notifications: {}", (data[0] & 0x01) != 0);
        }

        if handle == self.current.cccd_handle {
            //info!("battery current notifications: {}", (data[0] & 0x01) != 0);
        }

        if handle == self.capacity.cccd_handle {
            //info!("battery capacity notifications: {}", (data[0] & 0x01) != 0);
        }
    }
}


pub async fn run(connection: &Connection, server: &Server) {
    // TODO: add services here
    // do we need to mutpin?
    join::join4(
        update_level(connection, server), 
        update_power(connection, server), 
        update_voltage(connection, server), 
        update_current(connection, server),
        ).await;
}


pub async fn update_level(connection: &Connection, server: &Server) {
    let mut sub = signals::BATTERY_LEVEL.subscriber().unwrap();
    let handle = server.battery.level.value_handle;
    loop {
        let val = sub.next_message_pure().await;
        let _ = server::notify_value(connection, handle, &[val]);
    }
}


pub async fn update_power(connection: &Connection, server: &Server) {
    // let mut sub = signals::BATTERY_LEVEL.subscriber().unwrap();
    // let handle = server.battery.level.value_handle;
    // loop {
    //     let val = sub.next_message_pure().await;
    //     //let val = functions::bitshift_split_u16(val);
    //     let _ = server::notify_value(connection, handle, &[val]);
    // }
}


pub async fn update_voltage(connection: &Connection, server: &Server) {
    // let mut sub = signals::BATTERY_LEVEL.subscriber().unwrap();
    // let handle = server.battery.level.value_handle;
    // loop {
    //     let val = sub.next_message_pure().await;
    //     //let val = functions::bitshift_split_u16(val);
    //     let _ = server::notify_value(connection, handle, &[val]);
    // }
}


pub async fn update_current(connection: &Connection, server: &Server) {
    // let mut sub = signals::BATTERY_LEVEL.subscriber().unwrap();
    // let handle = server.battery.level.value_handle;
    // loop {
    //     let val = sub.next_message_pure().await;
    //     //let val = functions::bitshift_split_u16(val);
    //     let _ = server::notify_value(connection, handle, &[val]);
    // }
}