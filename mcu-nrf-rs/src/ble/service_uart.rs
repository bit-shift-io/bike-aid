use defmt::*;
use nrf_softdevice::ble::gatt_server::builder::ServiceBuilder;
use nrf_softdevice::ble::gatt_server::characteristic::{Attribute, Metadata, Properties};
use nrf_softdevice::ble::gatt_server::{CharacteristicHandles, RegisterError};
use nrf_softdevice::ble::{Connection, Uuid};
use nrf_softdevice::Softdevice;
use crate::utils::{functions, globals};
use crate::utils::signals;

// little endian, so reverse order for bytes!
// uart service: 6E400001-B5A3-F393-E0A9-E50E24DCCA9E
// rx characteristic: 6E400002-B5A3-F393-E0A9-E50E24DCCA9E
// tx characteristic: 6E400003-B5A3-F393-E0A9-E50E24DCCA9E
const UART_SERIVCE: [u8; 16] = [0x9E, 0xCA, 0xDC, 0x24, 0x0E, 0xE5, 0xA9, 0xE0, 0x93, 0xF3, 0xA3, 0xB5, 0x01, 0x00, 0x40, 0x6E];
const RX: [u8; 16] = [0x9E, 0xCA, 0xDC, 0x24, 0x0E, 0xE5, 0xA9, 0xE0, 0x93, 0xF3, 0xA3, 0xB5, 0x02, 0x00, 0x40, 0x6E];
const TX: [u8; 16] = [0x9E, 0xCA, 0xDC, 0x24, 0x0E, 0xE5, 0xA9, 0xE0, 0x93, 0xF3, 0xA3, 0xB5, 0x03, 0x00, 0x40, 0x6E];

pub struct UartService {
    pub rx: CharacteristicHandles,
    pub tx: CharacteristicHandles,
}


impl UartService {
    pub fn new(sd: &mut Softdevice) -> Result<Self, RegisterError> {
        let mut service_builder = ServiceBuilder::new(sd, Uuid::new_128(&UART_SERIVCE))?;

        let rx = service_builder.add_characteristic(
            Uuid::new_128(&RX),
            Attribute::new(&[]).variable_len(globals::BLE_BUFFER_LENGTH as u16),
            Metadata::new(Properties::new().write()), // .notify()
        )?;
        let rx_handle = rx.build();

        let tx = service_builder.add_characteristic(
            Uuid::new_128(&TX),
            Attribute::new(&[]).variable_len(globals::BLE_BUFFER_LENGTH as u16), 
            Metadata::new(Properties::new().notify().read()),
        )?;
        let tx_handle = tx.build();

        let _service_handle = service_builder.build();
        
        Ok(UartService {
            rx: rx_handle, // value_handle
            tx: tx_handle,
        })
    }


    pub fn on_write(&self, _connection: &Connection, handle: u16, data: &[u8]) {
        if handle == self.tx.cccd_handle {
            //info!("tx notifications: {}", (data[0] & 0x01) != 0);
        }

        if handle == self.rx.cccd_handle {
            //info!("rx notifications: {}", (data[0] & 0x01) != 0);
        }

        if handle == self.rx.value_handle {
            // recived data from uart
            info!("rx: {:?}", functions::bytes_to_string(data));
            let string = functions::byte_array_to_heapless_string(data);
            signals::UART_READ_WATCH.dyn_sender().send(string);
        }

        if handle == self.tx.value_handle {
            // recived data from uart
            info!("tx: {:?}", functions::bytes_to_string(data));
        }
    }
}