use nrf_softdevice::ble::gatt_server::builder::ServiceBuilder;
use nrf_softdevice::ble::gatt_server::characteristic::{Attribute, Metadata, Properties};
use nrf_softdevice::ble::gatt_server::{CharacteristicHandles, RegisterError};
use nrf_softdevice::ble::{Connection, Uuid};
use nrf_softdevice::Softdevice;
use crate::utils::globals;
use crate::utils::signals;

// little endian, so reverse order for bytes!
// uart service: 6E400001-B5A3-F393-E0A9-E50E24DCCA9E
// rx characteristic: 6E400002-B5A3-F393-E0A9-E50E24DCCA9E
// tx characteristic: 6E400003-B5A3-F393-E0A9-E50E24DCCA9E
const UUID_UART_SERIVCE: [u8; 16] = [0x9E, 0xCA, 0xDC, 0x24, 0x0E, 0xE5, 0xA9, 0xE0, 0x93, 0xF3, 0xA3, 0xB5, 0x01, 0x00, 0x40, 0x6E];
const UUID_RX: [u8; 16] = [0x9E, 0xCA, 0xDC, 0x24, 0x0E, 0xE5, 0xA9, 0xE0, 0x93, 0xF3, 0xA3, 0xB5, 0x02, 0x00, 0x40, 0x6E];
const UUID_TX: [u8; 16] = [0x9E, 0xCA, 0xDC, 0x24, 0x0E, 0xE5, 0xA9, 0xE0, 0x93, 0xF3, 0xA3, 0xB5, 0x03, 0x00, 0x40, 0x6E];

pub struct UartService {
    pub rx: CharacteristicHandles,
    pub tx: CharacteristicHandles,
}


impl UartService {
    pub fn new(sd: &mut Softdevice) -> Result<Self, RegisterError> {
        let mut service_builder = ServiceBuilder::new(sd, Uuid::new_128(&UUID_UART_SERIVCE))?;

        let cb = service_builder.add_characteristic(
            Uuid::new_128(&UUID_RX),
            Attribute::new(&[]).variable_len(globals::BUFFER_LENGTH as u16),
            Metadata::new(Properties::new().write()), // .notify()
        )?;
        let rx = cb.build();

        let cb = service_builder.add_characteristic(
            Uuid::new_128(&UUID_TX),
            Attribute::new(&[]).variable_len(globals::BUFFER_LENGTH as u16), 
            Metadata::new(Properties::new().notify().read()),
        )?;
        let tx = cb.build();

        let _service_handle = service_builder.build();
        
        Ok(UartService {
            rx, // value_handle
            tx,
        })
    }


    pub fn on_write(&self, _connection: &Connection, handle: u16, data: &[u8]) {
        match handle {
            handle if handle == self.rx.value_handle => {
                // send data to uart
                //info!("tx: {:?}", functions::bytes_to_string(data));
                signals::receive_uart(data);
            }
            _ => {}
        }
    }
}