use nrf_softdevice::ble::gatt_server::builder::ServiceBuilder;
use nrf_softdevice::ble::gatt_server::characteristic::{Attribute, Metadata, Properties};
use nrf_softdevice::ble::gatt_server::RegisterError;
use nrf_softdevice::ble::Uuid;
use nrf_softdevice::Softdevice;

// const SERIAL_NUMBER_STRING: &str = "007-awesome";
// const MODEL_NUMBER_STRING: &str = "diy-scooter";
// const FIRMWARE_REVISION_STRING: &str = "1.0";
// const HARDWARE_REVISION_STRING: &str = "1.0";
// const SOFTWARE_REVISION_STRING: &str = "1.0";
const MANUFACTURER_NAME_STRING: &str = "Bronson Mathews";
const EMAIL_STRING: &str = "bronsonmathews@gmail.com";

const DEVICE_INFORMATION: Uuid = Uuid::new_16(0x180a);
const EMAIL: Uuid = Uuid::new_16(0x2A87);
const MANUFACTURER_NAME: Uuid = Uuid::new_16(0x2a29);
// const MODEL_NUMBER: Uuid = Uuid::new_16(0x2a24);
// const SERIAL_NUMBER: Uuid = Uuid::new_16(0x2a25);
// const FIRMWARE_REVISION: Uuid = Uuid::new_16(0x2a26);
// const HARDWARE_REVISION: Uuid = Uuid::new_16(0x2a27);
// const SOFTWARE_REVISION: Uuid = Uuid::new_16(0x2a28);


// keep the fluff minimal to speed android app
pub struct DeviceInformationService {
    _manufacturer_name: u16,
    //_model_number: u16,
    //_serial_number: u16,
    //_hw_revision: u16,
    //_fw_revision: u16,
    //_sw_revision: u16,
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

        // let model_number = service_builder.add_characteristic(
        //     MODEL_NUMBER,
        //     Attribute::new(MODEL_NUMBER_STRING),
        //     Metadata::new(Properties::new().read()),
        // )?;
        // let model_number_handle = model_number.build();

        // let serial_number = service_builder.add_characteristic(
        //     SERIAL_NUMBER,
        //     Attribute::new(SERIAL_NUMBER_STRING),
        //     Metadata::new(Properties::new().read()),
        // )?;
        // let serial_number_handle = serial_number.build();

        // let hw_revision = service_builder.add_characteristic(
        //     HARDWARE_REVISION,
        //     Attribute::new(HARDWARE_REVISION_STRING),
        //     Metadata::new(Properties::new().read()),
        // )?;
        // let hw_revision_handle = hw_revision.build();

        // let fw_revision = service_builder.add_characteristic(
        //     FIRMWARE_REVISION,
        //     Attribute::new(FIRMWARE_REVISION_STRING),
        //     Metadata::new(Properties::new().read()),
        // )?;
        // let fw_revision_handle = fw_revision.build();

        // let sw_revision = service_builder.add_characteristic(
        //     SOFTWARE_REVISION,
        //     Attribute::new(SOFTWARE_REVISION_STRING),
        //     Metadata::new(Properties::new().read()),
        // )?;
        // let sw_revision_handle = sw_revision.build();

        let email = service_builder.add_characteristic(
            EMAIL,
            Attribute::new(EMAIL_STRING),
            Metadata::new(Properties::new().read()),
        )?;
        let email_handle = email.build();

        let _service_handle = service_builder.build();

        Ok(DeviceInformationService {
            _manufacturer_name: manufacturer_name_handle.value_handle,
            //_model_number: model_number_handle.value_handle,
            // _serial_number: serial_number_handle.value_handle,
            // _hw_revision: hw_revision_handle.value_handle,
            // _fw_revision: fw_revision_handle.value_handle,
            // _sw_revision: sw_revision_handle.value_handle,
            _email: email_handle.value_handle,
        })
    }

}

