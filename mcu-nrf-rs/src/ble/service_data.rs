use nrf_softdevice::ble::gatt_server::builder::ServiceBuilder;
use nrf_softdevice::ble::gatt_server::characteristic::{Attribute, Metadata, Properties};
use nrf_softdevice::ble::gatt_server::{CharacteristicHandles, RegisterError};
use nrf_softdevice::ble::Uuid;
use nrf_softdevice::Softdevice;

const UUID_DATA_SERVICE: Uuid = Uuid::new_16(0x2000);
const UUID_SPEED: Uuid = Uuid::new_16(0x2001);
const UUID_ODOMETER: Uuid = Uuid::new_16(0x2003);
const UUID_TEMPERATURE: Uuid = Uuid::new_16(0x2004);
const UUID_CLOCK_MINUTES: Uuid = Uuid::new_16(0x2005);
const UUID_CLOCK_HOURS: Uuid = Uuid::new_16(0x2006);
const UUID_BRAKE_ON: Uuid = Uuid::new_16(0x2007);
const UUID_PARK_BRAKE_ON: Uuid = Uuid::new_16(0x2008);
const UUID_CRUISE_LEVEL: Uuid = Uuid::new_16(0x2009);
const UUID_THROTTLE_LEVEL: Uuid = Uuid::new_16(0x2002);

pub struct DataService {
    pub speed: CharacteristicHandles,
    pub odometer: CharacteristicHandles,
    pub temperature: CharacteristicHandles,
    pub clock_minutes: CharacteristicHandles,
    pub clock_hours: CharacteristicHandles,
    pub brake_on: CharacteristicHandles,
    pub park_brake_on: CharacteristicHandles,
    pub cruise_level: CharacteristicHandles,
    pub throttle_level: CharacteristicHandles,
}

impl DataService {
    pub fn new(sd: &mut Softdevice) -> Result<Self, RegisterError> {
        let mut service_builder = ServiceBuilder::new(sd, UUID_DATA_SERVICE)?;

        let cb = service_builder.add_characteristic(
            UUID_SPEED,
            Attribute::new(&[0u8]),
            Metadata::new(Properties::new().read().notify())
        )?;
        let speed = cb.build();

        let cb = service_builder.add_characteristic(
            UUID_ODOMETER,
            Attribute::new(&[0u8]),
            Metadata::new(Properties::new().read().notify()),
        )?;
        let odometer = cb.build();

        let cb = service_builder.add_characteristic(
            UUID_TEMPERATURE,
            Attribute::new(&[0u8; 2]),
            Metadata::new(Properties::new().read().notify()),
        )?;
        let temperature = cb.build();

        let cb = service_builder.add_characteristic(
            UUID_CLOCK_MINUTES,
            Attribute::new(&[0u8]),
            Metadata::new(Properties::new().read().notify()),
        )?;
        let clock_minutes = cb.build();

        let cb = service_builder.add_characteristic(
            UUID_CLOCK_HOURS,
            Attribute::new(&[0u8]),
            Metadata::new(Properties::new().read().notify()),
        )?;
        let clock_hours = cb.build();

        let cb = service_builder.add_characteristic(
            UUID_BRAKE_ON,
            Attribute::new(&[0u8]),
            Metadata::new(Properties::new().read().notify()),
        )?;
        let brake_on = cb.build();

        let cb = service_builder.add_characteristic(
            UUID_PARK_BRAKE_ON,
            Attribute::new(&[1u8]), // default true
            Metadata::new(Properties::new().read().notify()),
        )?;
        let park_brake_on = cb.build();

        let cb = service_builder.add_characteristic(
            UUID_CRUISE_LEVEL,
            Attribute::new(&[0u8]),
            Metadata::new(Properties::new().read().notify()),
        )?;
        let cruise_level = cb.build();

        let cb = service_builder.add_characteristic(
            UUID_THROTTLE_LEVEL,
            Attribute::new(&[0u8; 2]),
            Metadata::new(Properties::new().read().notify()),
        )?;
        let throttle_level = cb.build();

        let _service_handle = service_builder.build();
        
        Ok(DataService {
            speed,
            odometer,
            temperature,
            clock_minutes,
            clock_hours,
            brake_on,
            park_brake_on,
            cruise_level,
            throttle_level
        })
    }
}