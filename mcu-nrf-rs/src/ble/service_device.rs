use nrf_softdevice::ble::gatt_server::builder::ServiceBuilder;
use nrf_softdevice::ble::gatt_server::characteristic::{Attribute, Metadata, Properties};
use nrf_softdevice::ble::gatt_server::RegisterError;
use nrf_softdevice::ble::Uuid;
use nrf_softdevice::Softdevice;

const MANUFACTURER_NAME_STRING: &str = "Bronson Mathews";
const EMAIL_STRING: &str = "bronsonmathews@gmail.com";
const DEVICE_INFORMATION: Uuid = Uuid::new_16(0x180a);
const EMAIL: Uuid = Uuid::new_16(0x2a87);
const MANUFACTURER_NAME: Uuid = Uuid::new_16(0x2a29);

pub struct DeviceInformationService {
    _manufacturer_name: u16,
    _email: u16,
}


impl DeviceInformationService {
    pub fn new(sd: &mut Softdevice) -> Result<Self, RegisterError> {
        let mut service_builder = ServiceBuilder::new(sd, DEVICE_INFORMATION)?;

        let manufacturer_name = service_builder.add_characteristic(
            MANUFACTURER_NAME,
            Attribute::new(MANUFACTURER_NAME_STRING),
            Metadata::new(Properties::new().read()),
        )?;
        let manufacturer_name_handle = manufacturer_name.build();

        let email = service_builder.add_characteristic(
            EMAIL,
            Attribute::new(EMAIL_STRING),
            Metadata::new(Properties::new().read()),
        )?;
        let email_handle = email.build();

        let _service_handle = service_builder.build();

        Ok(DeviceInformationService {
            _manufacturer_name: manufacturer_name_handle.value_handle,
            _email: email_handle.value_handle,
        })
    }
}