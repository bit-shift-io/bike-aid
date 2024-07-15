use defmt::info;
use nrf_softdevice::ble::gatt_server::builder::ServiceBuilder;
use nrf_softdevice::ble::gatt_server::characteristic::{Attribute, Metadata, Properties};
use nrf_softdevice::ble::gatt_server::RegisterError;
use nrf_softdevice::ble::Uuid;
use nrf_softdevice::Softdevice;

pub struct UARTService {
    rx: u16,
    tx: u16,
}

impl UARTService {
    pub fn new(sd: &mut Softdevice) -> Result<Self, RegisterError> {
        // https://learn.adafruit.com/introducing-adafruit-ble-bluetooth-low-energy-friend/uart-service
        // 6E400001-B5A3-F393-E0A9-E50E24DCCA9E
        let service_id: Uuid = Uuid::new_128(&[
            0x6E, 0x40, 0x00, 0x01, 0xB5, 0xA3, 0xF3, 0x93,
            0xE0, 0xA9, 0xE5, 0x0E, 0x24, 0xDC, 0xCA, 0x9E,
        ]);
        const RX: Uuid = Uuid::new_16(0x0002); // "6E400002-B5A3-F393-E0A9-E50E24DCCA9E"
        const TX: Uuid = Uuid::new_16(0x0003); // "6E400003-B5A3-F393-E0A9-E50E24DCCA9E"

        let mut service_builder = ServiceBuilder::new(sd, service_id)?;

        let rx = service_builder.add_characteristic(
            RX,
            Attribute::new([0x11u8, 0x1u8]),
            Metadata::new(Properties::new().read().notify()),
        )?;
        let rx_handle = rx.build();

        let tx = service_builder.add_characteristic(
            TX,
            Attribute::new([0x22u8, 0x2u8]),
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
            info!("rx: {:?}", data);
        }

        if handle == self.tx {
            info!("tx: {:?}", data);
        }
    }
}

