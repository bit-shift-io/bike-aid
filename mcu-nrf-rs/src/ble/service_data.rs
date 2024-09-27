use super::server::{self, *};
use crate::utils::{functions, signals};
use defmt::*;
use nrf_softdevice::ble::gatt_server::builder::ServiceBuilder;
use nrf_softdevice::ble::gatt_server::characteristic::{Attribute, Metadata, Properties};
use nrf_softdevice::ble::gatt_server::{CharacteristicHandles, RegisterError};
use nrf_softdevice::ble::{Connection, Uuid};
use nrf_softdevice::Softdevice;
use embassy_futures::join;
use futures::join;

const SERVICE_ID: Uuid = Uuid::new_16(0x2000);
const SPEED: Uuid = Uuid::new_16(0x2001);
const TRIP_DURATION: Uuid = Uuid::new_16(0x2002);
const ODOMETER: Uuid = Uuid::new_16(0x2003);
const TEMPERATURE: Uuid = Uuid::new_16(0x2004);
const CLOCK_MINUTES: Uuid = Uuid::new_16(0x2005);
const CLOCK_HOURS: Uuid = Uuid::new_16(0x2006);

pub struct DataService {
    speed: CharacteristicHandles,
    trip_duration: CharacteristicHandles,
    odometer: CharacteristicHandles,
    temperature: CharacteristicHandles,
    clock_minutes: CharacteristicHandles,
    clock_hours: CharacteristicHandles,
}

impl DataService {
    pub fn new(sd: &mut Softdevice) -> Result<Self, RegisterError> {
        let mut service_builder = ServiceBuilder::new(sd, SERVICE_ID)?;

        let characteristic_builder = service_builder.add_characteristic(
            SPEED,
            Attribute::new(&[0u8]),
            Metadata::new(Properties::new().read().notify())
        )?;
        let speed_handle = characteristic_builder.build();

        let characteristic_builder = service_builder.add_characteristic(
            TRIP_DURATION,
            Attribute::new(&[0u8]),
            Metadata::new(Properties::new().read().notify()),
        )?;
        let trip_duration_handle = characteristic_builder.build();

        let characteristic_builder = service_builder.add_characteristic(
            ODOMETER,
            Attribute::new(&[0u8]),
            Metadata::new(Properties::new().read().notify()),
        )?;
        let odometer_handle = characteristic_builder.build();

        let characteristic_builder = service_builder.add_characteristic(
            TEMPERATURE,
            Attribute::new(&[0u8; 2]),
            Metadata::new(Properties::new().read().notify()),
        )?;
        let temperature_handle = characteristic_builder.build();

        let characteristic_builder = service_builder.add_characteristic(
            CLOCK_MINUTES,
            Attribute::new(&[0u8]),
            Metadata::new(Properties::new().read().notify()),
        )?;
        let clock_minutes_handle = characteristic_builder.build();

        let characteristic_builder = service_builder.add_characteristic(
            CLOCK_HOURS,
            Attribute::new(&[0u8]),
            Metadata::new(Properties::new().read().notify()),
        )?;
        let clock_hours_handle = characteristic_builder.build();

        let _service_handle = service_builder.build();
        
        Ok(DataService {
            speed: speed_handle,
            trip_duration: trip_duration_handle,
            odometer: odometer_handle,
            temperature: temperature_handle,
            clock_minutes: clock_minutes_handle,
            clock_hours: clock_hours_handle,
        })
    }


    pub fn on_write(&self, _connection: &Connection, handle: u16, data: &[u8]) {
        if data.is_empty() {
            return;
        }

        if handle == self.clock_hours.cccd_handle {
            info!("clock notifications: {}", (data[0] & 0x01) != 0);
        }
    }
}


pub async fn run(connection: &Connection, server: &Server) {
    /*
    speed: CharacteristicHandles,
    trip_duration: CharacteristicHandles,
    odometer: CharacteristicHandles,
    temperature: CharacteristicHandles,
    clock_minutes: CharacteristicHandles,
    clock_hours: CharacteristicHandles,
     */
    // TODO: add services here
    // do we need to mutpin?
    futures::join!(
        update_odometer(connection, server),
        update_speed(connection, server), 
        update_temperature(connection, server), 
        update_clock_minutes(connection, server), 
        update_clock_hours(connection, server),
    );
    /*
    join::join4(
        update_speed(connection, server), 
        update_temperature(connection, server), 
        update_clock_minutes(connection, server), 
        update_clock_hours(connection, server),
        ).await;
     */
}


pub async fn update_odometer(connection: &Connection, server: &Server) {
    let mut sub = signals::ODOMETER.subscriber().unwrap();
    let handle = server.data.odometer.value_handle;
    loop {
        let val = sub.next_message_pure().await;
        //let val = functions::bitshift_split_u16(val);

        let bytes: [u8; 2] = val.to_le_bytes(); // Convert to little-endian byte array
        // If you want a slice
        //let val: &[u8] = &bytes;

        // TODO: crash here, panic
        //let _ = server::notify_value(connection, handle, &bytes);
    }
}


pub async fn update_speed(connection: &Connection, server: &Server) {
    let mut sub = signals::SMOOTH_SPEED.subscriber().unwrap();
    let handle = server.data.speed.value_handle;
    loop {
        let val = sub.next_message_pure().await;
        //let val = functions::bitshift_split_u16(val);
        let _ = server::notify_value(connection, handle, &[val]);
    }
}


pub async fn update_temperature(connection: &Connection, server: &Server) {
    let mut sub = signals::TEMPERATURE.subscriber().unwrap();
    let handle = server.data.temperature.value_handle;
    loop {
        let val = sub.next_message_pure().await;
        let _ = server::notify_value(connection, handle, &[val]);
    }
}


pub async fn update_clock_minutes(connection: &Connection, server: &Server) {
    let mut sub = signals::CLOCK_MINUTES.subscriber().unwrap();
    let handle = server.data.clock_minutes.value_handle;
    loop {
        let val = sub.next_message_pure().await;
        let _ = server::notify_value(connection, handle, &[val]);
    }
}


pub async fn update_clock_hours(connection: &Connection, server: &Server) {
    let mut sub = signals::CLOCK_HOURS.subscriber().unwrap();
    let handle = server.data.clock_hours.value_handle;
    loop {
        let val = sub.next_message_pure().await;
        let _ = server::notify_value(connection, handle, &[val]);
    }
}