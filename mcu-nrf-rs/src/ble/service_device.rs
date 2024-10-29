use nrf_softdevice::ble::gatt_server::builder::ServiceBuilder;
use nrf_softdevice::ble::gatt_server::characteristic::{Attribute, Metadata, Properties};
use nrf_softdevice::ble::gatt_server::RegisterError;
use nrf_softdevice::ble::Uuid;
use nrf_softdevice::Softdevice;

const MANUFACTURER_NAME: &str = "Bronson Mathews";
const EMAIL: &str = "bronsonmathews@gmail.com";
const UUID_DEVICE_INFORMATION_SERVICE: Uuid = Uuid::new_16(0x180a);
const UUID_EMAIL: Uuid = Uuid::new_16(0x2a87);
const UUID_MANUFACTURER_NAME: Uuid = Uuid::new_16(0x2a29);
const UUID_FIRMWARE_REVISION: Uuid = Uuid::new_16(0x2a26);

pub struct DeviceInformationService {
    _manufacturer_name: u16,
    _email: u16,
    _firmware_revision: u16,
}


impl DeviceInformationService {
    pub fn new(sd: &mut Softdevice) -> Result<Self, RegisterError> {
        let mut service_builder = ServiceBuilder::new(sd, UUID_DEVICE_INFORMATION_SERVICE)?;

        let manufacturer_name = service_builder.add_characteristic(
            UUID_MANUFACTURER_NAME,
            Attribute::new(MANUFACTURER_NAME),
            Metadata::new(Properties::new().read()),
        )?;
        let manufacturer_name_handle = manufacturer_name.build();

        let email = service_builder.add_characteristic(
            UUID_EMAIL,
            Attribute::new(EMAIL),
            Metadata::new(Properties::new().read()),
        )?;
        let email_handle = email.build();

        let _firmware_revision = service_builder.add_characteristic(
            UUID_FIRMWARE_REVISION,
            Attribute::new(&[0u8]),
            Metadata::new(Properties::new().read()),
        )?;
        let _firmware_revision_handle = _firmware_revision.build();

        let _service_handle = service_builder.build();

        Ok(DeviceInformationService {
            _manufacturer_name: manufacturer_name_handle.value_handle,
            _email: email_handle.value_handle,
            _firmware_revision: _firmware_revision_handle.value_handle,
        })
    }
}