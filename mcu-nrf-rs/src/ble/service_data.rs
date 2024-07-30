use super::server::{self, *};
use crate::utils::functions;
use crate::utils::signals;
use defmt::*;
use nrf_softdevice::ble::gatt_server::builder::ServiceBuilder;
use nrf_softdevice::ble::gatt_server::characteristic::{Attribute, Metadata, Properties};
use nrf_softdevice::ble::gatt_server::{self, CharacteristicHandles, RegisterError};
use nrf_softdevice::ble::{Connection, Uuid};
use nrf_softdevice::Softdevice;

const SERVICE_ID: Uuid = Uuid::new_16(0x2000);
const SPEED: Uuid = Uuid::new_16(0x2001);
const TRIP_DURATION: Uuid = Uuid::new_16(0x2002);
const ODOMETER: Uuid = Uuid::new_16(0x2003);
const TEMPERATURE: Uuid = Uuid::new_16(0x2004);
const CLOCK_MINUTES: Uuid = Uuid::new_16(0x1005);
const CLOCK_HOURS: Uuid = Uuid::new_16(0x1006);

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
            Attribute::new(&[0u8; 16]),
            Metadata::new(Properties::new().read().notify())
        )?;
        let speed_handle = characteristic_builder.build();

        let characteristic_builder = service_builder.add_characteristic(
            TRIP_DURATION,
            Attribute::new(&[0u8; 2]),
            Metadata::new(Properties::new().read().notify()),
        )?;
        let trip_duration_handle = characteristic_builder.build();

        let characteristic_builder = service_builder.add_characteristic(
            ODOMETER,
            Attribute::new(&[0u8; 2]),
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
            Attribute::new(&[0u8; 2]),
            Metadata::new(Properties::new().read().notify()),
        )?;
        let clock_minutes_handle = characteristic_builder.build();

        let characteristic_builder = service_builder.add_characteristic(
            CLOCK_HOURS,
            Attribute::new(&[0u8; 2]),
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


    pub fn speed_set(&self, val: &i16) -> Result<(), gatt_server::SetValueError> {
        let split = functions::bitshift_split_u16(*val);
        server::set_value(self.speed.value_handle, &split)
    }
    

    pub fn speed_notify(&self, conn: &Connection, val: &i16) -> Result<(), gatt_server::NotifyValueError> {
        let split = functions::bitshift_split_u16(*val);
        server::notify_value(conn, self.speed.value_handle, &split)
    }
}

pub async fn run(connection: &Connection, server: &Server) {
    // TODO: add data points here
    let mut sub_throttle_in = signals::THROTTLE_IN.subscriber().unwrap();

    loop {
        let val = sub_throttle_in.next_message_pure().await;

        // try notify, if fails due to other device not allowing, then just set the data
        match server.data.speed_notify(connection, &val) {
            Ok(_) => (),
            Err(_) => unwrap!(server.data.speed_set(&val)),
        };
    }
}