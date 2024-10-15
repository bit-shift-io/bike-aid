use super::server::{self, *};
use crate::utils::signals;
use nrf_softdevice::ble::gatt_server::builder::ServiceBuilder;
use nrf_softdevice::ble::gatt_server::characteristic::{Attribute, Metadata, Properties};
use nrf_softdevice::ble::gatt_server::{CharacteristicHandles, RegisterError};
use nrf_softdevice::ble::{Connection, Uuid};
use nrf_softdevice::Softdevice;

const SERVICE_ID: Uuid = Uuid::new_16(0x2000);
const SPEED: Uuid = Uuid::new_16(0x2001);
const ODOMETER: Uuid = Uuid::new_16(0x2003);
const TEMPERATURE: Uuid = Uuid::new_16(0x2004);
const CLOCK_MINUTES: Uuid = Uuid::new_16(0x2005);
const CLOCK_HOURS: Uuid = Uuid::new_16(0x2006);
const BRAKE_ON: Uuid = Uuid::new_16(0x2007);
const PARK_BRAKE_ON: Uuid = Uuid::new_16(0x2008);
const CRUISE_LEVEL: Uuid = Uuid::new_16(0x2009);

pub struct DataService {
    speed: CharacteristicHandles,
    odometer: CharacteristicHandles,
    temperature: CharacteristicHandles,
    clock_minutes: CharacteristicHandles,
    clock_hours: CharacteristicHandles,
    brake_on: CharacteristicHandles,
    park_brake_on: CharacteristicHandles,
    cruise_level: CharacteristicHandles,
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

        let characteristic_builder = service_builder.add_characteristic(
            BRAKE_ON,
            Attribute::new(&[0u8]),
            Metadata::new(Properties::new().read().notify()),
        )?;
        let brake_on_handle = characteristic_builder.build();

        let characteristic_builder = service_builder.add_characteristic(
            PARK_BRAKE_ON,
            Attribute::new(&[0u8]),
            Metadata::new(Properties::new().read().notify()),
        )?;
        let park_brake_on_handle = characteristic_builder.build();

        let characteristic_builder = service_builder.add_characteristic(
            CRUISE_LEVEL,
            Attribute::new(&[0u8]),
            Metadata::new(Properties::new().read().notify()),
        )?;
        let cruise_level_handle = characteristic_builder.build();

        let _service_handle = service_builder.build();
        
        Ok(DataService {
            speed: speed_handle,
            odometer: odometer_handle,
            temperature: temperature_handle,
            clock_minutes: clock_minutes_handle,
            clock_hours: clock_hours_handle,
            brake_on: brake_on_handle,
            park_brake_on: park_brake_on_handle,
            cruise_level: cruise_level_handle,
        })
    }


    pub fn on_write(&self, _connection: &Connection, handle: u16, data: &[u8]) {
        if data.is_empty() {
            return;
        }

        if handle == self.clock_hours.cccd_handle {
            //info!("clock notifications: {}", (data[0] & 0x01) != 0);
        }
    }
}


pub async fn run(connection: &Connection, server: &Server) {
    // TODO: add services here
    // do we need to mutpin?
    futures::join!(
        update_odometer(connection, server),
        update_speed(connection, server), 
        update_temperature(connection, server), 
        update_clock_minutes(connection, server), 
        update_clock_hours(connection, server),
        update_brake_on(connection, server),
        update_park_brake_on(connection, server),
        update_cruise_level(connection, server),
    );
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
    let mut rec = signals::TEMPERATURE_WATCH.receiver().unwrap();
    let handle = server.data.temperature.value_handle;
    loop {
        let val = rec.changed().await;
        let _ = server::notify_value(connection, handle, &[val]);
    }
}


pub async fn update_clock_minutes(connection: &Connection, server: &Server) {
    let mut rec = signals::CLOCK_MINUTES_WATCH.receiver().unwrap();
    let handle = server.data.clock_minutes.value_handle;
    loop {
        let val = rec.changed().await;
        let _ = server::notify_value(connection, handle, &[val]);
    }
}


pub async fn update_clock_hours(connection: &Connection, server: &Server) {
    //let mut sub = signals::CLOCK_HOURS.subscriber().unwrap();
    let mut rec = signals::CLOCK_HOURS_WATCH.receiver().unwrap();
    let handle = server.data.clock_hours.value_handle;
    loop {
        let val = rec.changed().await;
        let _ = server::notify_value(connection, handle, &[val]);
    }
}


pub async fn update_brake_on(connection: &Connection, server: &Server) {
    let mut rec = signals::BRAKE_ON_WATCH.receiver().unwrap();
    let handle = server.data.brake_on.value_handle;
    loop {
        let val = rec.changed().await;
        let _ = server::notify_value(connection, handle, &[val as u8]);
    }
}


pub async fn update_park_brake_on(connection: &Connection, server: &Server) {
    let mut rec = signals::PARK_BRAKE_ON_WATCH.receiver().unwrap();
    let handle = server.data.park_brake_on.value_handle;
    loop {
        let val = rec.changed().await;
        let _ = server::notify_value(connection, handle, &[val as u8]);
    }
}


pub async fn update_cruise_level(connection: &Connection, server: &Server) {
    let mut rec = signals::CRUISE_LEVEL_WATCH.receiver().unwrap();
    let handle = server.data.cruise_level.value_handle;
    loop {
        let val = rec.changed().await;
        let _ = server::notify_value(connection, handle, &[val]);
    }
}