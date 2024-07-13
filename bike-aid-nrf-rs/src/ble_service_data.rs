use defmt::info;
use nrf_softdevice::ble::gatt_server::builder::ServiceBuilder;
use nrf_softdevice::ble::gatt_server::characteristic::{Attribute, Metadata, Properties};
use nrf_softdevice::ble::gatt_server::{self, RegisterError};
use nrf_softdevice::ble::{Connection, Uuid};
use nrf_softdevice::Softdevice;

const SERVICE_ID: Uuid = Uuid::new_16(2000u16);
const THROTTLE_INPUT_VOLTAGE: Uuid = Uuid::new_16(20001u16);
const THROTTLE_OUTPUT_VOLTAGE: Uuid = Uuid::new_16(20002u16);

pub struct DataService {
    throttle_input_voltage: u16,
    throttle_output_voltage: u16,
}

impl DataService {
    pub fn new(sd: &mut Softdevice) -> Result<Self, RegisterError> {
        let mut service_builder = ServiceBuilder::new(sd, SERVICE_ID)?;

        let true_u8 = true as u8;
        let false_u8 = false as u8;

        let throttle_input_voltage = service_builder.add_characteristic(
            THROTTLE_INPUT_VOLTAGE,
            Attribute::new([true_u8]),
            Metadata::new(Properties::new().read().notify()),
        )?;
        let throttle_input_voltage_handle = throttle_input_voltage.build();

        let throttle_output_voltage = service_builder.add_characteristic(
            THROTTLE_OUTPUT_VOLTAGE,
            Attribute::new([true_u8]),
            Metadata::new(Properties::new().read().notify()),
        )?;
        let throttle_output_voltage_handle = throttle_output_voltage.build();

        let _service_handle = service_builder.build();
        
        Ok(DataService {
            throttle_input_voltage: throttle_input_voltage_handle.value_handle,
            throttle_output_voltage: throttle_output_voltage_handle.value_handle,
        })
    }

    pub fn throttle_input_voltage_set(&self, sd: &Softdevice, val: u8) -> Result<(), gatt_server::SetValueError> {
        gatt_server::set_value(sd, self.throttle_input_voltage, &[val])
    }
    
    pub fn throttle_input_voltage_notify(&self, conn: &Connection, val: u8) -> Result<(), gatt_server::NotifyValueError> {
        gatt_server::notify_value(conn, self.throttle_input_voltage, &[val])
    }

}

