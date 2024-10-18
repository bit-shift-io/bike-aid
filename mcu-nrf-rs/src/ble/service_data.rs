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
    pub speed: CharacteristicHandles,
    pub odometer: CharacteristicHandles,
    pub temperature: CharacteristicHandles,
    pub clock_minutes: CharacteristicHandles,
    pub clock_hours: CharacteristicHandles,
    pub brake_on: CharacteristicHandles,
    pub park_brake_on: CharacteristicHandles,
    pub cruise_level: CharacteristicHandles,
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
            Attribute::new(&[1u8]), // default true
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