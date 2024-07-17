use defmt::{info, unwrap};
use nrf_softdevice::ble::gatt_server::builder::ServiceBuilder;
use nrf_softdevice::ble::gatt_server::characteristic::{Attribute, Metadata, Properties};
use nrf_softdevice::ble::gatt_server::{self, RegisterError};
use nrf_softdevice::ble::{Connection, Uuid};
use nrf_softdevice::Softdevice;

use crate::ble_server::{self, Server};
use crate::signals;

pub struct UARTService {
    rx: u16,
    tx: u16,
}

impl UARTService {
    pub fn new(sd: &mut Softdevice) -> Result<Self, RegisterError> {
        // https://learn.adafruit.com/introducing-adafruit-ble-bluetooth-low-energy-friend/uart-service
        // little endian, so reverse order for bytes!

        // uart service: 6E400001-B5A3-F393-E0A9-E50E24DCCA9E
        // rx characteristic: 6E400002-B5A3-F393-E0A9-E50E24DCCA9E
        // tx characteristic: 6E400003-B5A3-F393-E0A9-E50E24DCCA9E
        const UART_SERIVCE: [u8; 16] = [0x9E, 0xCA, 0xDC, 0x24, 0x0E, 0xE5, 0xA9, 0xE0, 0x93, 0xF3, 0xA3, 0xB5, 0x01, 0x00, 0x40, 0x6E];
        const RX: [u8; 16] = [0x9E, 0xCA, 0xDC, 0x24, 0x0E, 0xE5, 0xA9, 0xE0, 0x93, 0xF3, 0xA3, 0xB5, 0x02, 0x00, 0x40, 0x6E];
        const TX: [u8; 16] = [0x9E, 0xCA, 0xDC, 0x24, 0x0E, 0xE5, 0xA9, 0xE0, 0x93, 0xF3, 0xA3, 0xB5, 0x03, 0x00, 0x40, 0x6E];

        let service_id: Uuid = Uuid::new_128(&UART_SERIVCE);
        let rx_characteristic: Uuid = Uuid::new_128(&RX);
        let tx_characteristic: Uuid = Uuid::new_128(&TX);

        let mut service_builder = ServiceBuilder::new(sd, service_id)?;

        let rx = service_builder.add_characteristic(
            rx_characteristic,
            Attribute::new([0x11u8, 0x1u8]), // TODO: need bigger length for text string
            Metadata::new(Properties::new().read().notify()),
        )?;
        let rx_handle = rx.build();

        let tx = service_builder.add_characteristic(
            tx_characteristic,
            Attribute::new([0x22u8, 0x2u8]), // TODO: need bigger length for text string
            Metadata::new(Properties::new().write()),
        )?;
        let tx_handle = tx.build();

        let _service_handle = service_builder.build();
        
        Ok(UARTService {
            rx: rx_handle.value_handle,
            tx: tx_handle.value_handle,
        })
    }


    pub fn on_write(&self, handle: u16, data: &[u8]) {
        if data.is_empty() {
            return;
        }

        if handle == self.rx {
            // unused, rx is send only
            info!("rx: {:?}", data);
        }

        if handle == self.tx {
            // recived data from uart
                // Convert the byte array to a string
            let byte_slice = &data;
            let string = core::str::from_utf8(byte_slice).unwrap();
            info!("tx: {}", string);
        }
    }

    // bypassed gatt_server
    pub fn rx_set(&self, val: &[u8]) -> Result<(), gatt_server::SetValueError> {
        ble_server::set_value(self.rx, &val)
    }
    
    // bypassed gatt_server
    pub fn rx_notify(&self, conn: &Connection, val: &[u8]) -> Result<(), gatt_server::NotifyValueError> {
        ble_server::notify_value(conn, self.rx, &val)
    }
}


pub async fn run(connection: &Connection, server: &Server) {
    let mut sub_rx = signals::UART_RX.subscriber().unwrap();

    loop {
        let val = sub_rx.next_message_pure().await;

        // try notify, if fails due to other device not allowing, then just set the data
        match server.uart.rx_notify(connection, &val) {
            Ok(_) => (),
            Err(_) => unwrap!(server.uart.rx_set(&val)),
        };
    }
}

