use defmt::info;
use nrf_softdevice::ble::gatt_server::builder::ServiceBuilder;
use nrf_softdevice::ble::gatt_server::characteristic::{Attribute, Metadata, Properties};
use nrf_softdevice::ble::gatt_server::RegisterError;
use nrf_softdevice::ble::Uuid;
use nrf_softdevice::Softdevice;

const SERVICE_ID: Uuid = Uuid::new_16(0x5000);
const RX: Uuid = Uuid::new_16(0x5001);
const TX: Uuid = Uuid::new_16(0x5002);

pub struct UARTService {
    rx: u16,
    tx: u16,
}

impl UARTService {
    pub fn new(sd: &mut Softdevice) -> Result<Self, RegisterError> {
        let mut service_builder = ServiceBuilder::new(sd, SERVICE_ID)?;

        let rx = service_builder.add_characteristic(
            RX,
            Attribute::new([0x11u8, 0x1u8]),
            Metadata::new(Properties::new().read().write()),
        )?;
        let rx_handle = rx.build();

        let tx = service_builder.add_characteristic(
            TX,
            Attribute::new([0x22u8, 0x2u8]),
            Metadata::new(Properties::new().read().write()),
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

