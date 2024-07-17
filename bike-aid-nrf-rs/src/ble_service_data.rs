use crate::ble_server::{self, *};
use crate::functions::shift_split_u16;
use crate::signals;
use defmt::*;
use nrf_softdevice::ble::gatt_server::builder::ServiceBuilder;
use nrf_softdevice::ble::gatt_server::characteristic::{Attribute, Metadata, Properties};
use nrf_softdevice::ble::gatt_server::{self, RegisterError};
use nrf_softdevice::ble::{Connection, Uuid};
use nrf_softdevice::Softdevice;

const SERVICE_ID: Uuid = Uuid::new_16(0x2000);
const THROTTLE_INPUT_VOLTAGE: Uuid = Uuid::new_16(0x2001);
const THROTTLE_OUTPUT_VOLTAGE: Uuid = Uuid::new_16(0x2002);

pub struct DataService {
    throttle_input_voltage: u16,
    throttle_output_voltage: u16,
}

impl DataService {
    pub fn new(sd: &mut Softdevice) -> Result<Self, RegisterError> {
        let mut service_builder = ServiceBuilder::new(sd, SERVICE_ID)?;

        /*
        // TODO: display units
        let metadata = Metadata::new(Properties::new().read().notify()).presentation(Presentation {
            format: raw::BLE_GATT_CPF_FORMAT_UINT8 as u8, // unsigned uint 8
            exponent: 0,  /* Value * 10 ^ 0 */
            unit: 0x27AD, /* Percentage */
            name_space: raw::BLE_GATT_CPF_NAMESPACE_BTSIG as u8, // assigned by Bluetooth SIG
            description: raw::BLE_GATT_CPF_NAMESPACE_DESCRIPTION_UNKNOWN as u16, // unknown
        });
         */

        let throttle_input_voltage = service_builder.add_characteristic(
            THROTTLE_INPUT_VOLTAGE,
            Attribute::new([0u8; 16]),
            Metadata::new(Properties::new().read().notify())
        )?;
        let throttle_input_voltage_handle = throttle_input_voltage.build();

        let throttle_output_voltage = service_builder.add_characteristic(
            THROTTLE_OUTPUT_VOLTAGE,
            Attribute::new([0x11u8, 0x1u8]),
            Metadata::new(Properties::new().read().notify()),
        )?;
        let throttle_output_voltage_handle = throttle_output_voltage.build();

        let _service_handle = service_builder.build();
        
        Ok(DataService {
            throttle_input_voltage: throttle_input_voltage_handle.value_handle,
            throttle_output_voltage: throttle_output_voltage_handle.value_handle,
        })
    }

    // bypassed gatt_server
    pub fn throttle_input_voltage_set(&self, val: &i16) -> Result<(), gatt_server::SetValueError> {
        let split = shift_split_u16(*val);
        ble_server::set_value(self.throttle_input_voltage, &split)
    }
    
    // bypassed gatt_server
    pub fn throttle_input_voltage_notify(&self, conn: &Connection, val: &i16) -> Result<(), gatt_server::NotifyValueError> {
        let split = shift_split_u16(*val);
        ble_server::notify_value(conn, self.throttle_input_voltage, &split)
    }
}

pub async fn run(connection: &Connection, server: &Server) {
    let mut sub_throttle_in = signals::THROTTLE_IN.subscriber().unwrap();

    loop {
        let val = sub_throttle_in.next_message_pure().await;

        // try notify, if fails due to other device not allowing, then just set the data
        match server.data.throttle_input_voltage_notify(connection, &val) {
            Ok(_) => (),
            Err(_) => unwrap!(server.data.throttle_input_voltage_set(&val)),
        };
    }
}