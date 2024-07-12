use defmt::info;
use nrf_softdevice::ble::gatt_server::builder::ServiceBuilder;
use nrf_softdevice::ble::gatt_server::characteristic::{Attribute, Metadata, Properties};
use nrf_softdevice::ble::gatt_server::RegisterError;
use nrf_softdevice::ble::Uuid;
use nrf_softdevice::Softdevice;

const SERVICE_ID: Uuid = Uuid::new_16(1000u16);
const RX: Uuid = Uuid::new_16(10001u16);
const TX: Uuid = Uuid::new_16(10001u16);

pub struct DataService {
    rx: u16,
    tx: u16,
}

impl DataService {
    pub fn new(sd: &mut Softdevice) -> Result<Self, RegisterError> {
        let mut service_builder = ServiceBuilder::new(sd, SERVICE_ID)?;

        let true_u8 = true as u8;
        let false_u8 = false as u8;

        let power_switch = service_builder.add_characteristic(
            RX,
            Attribute::new([true_u8]),
            Metadata::new(Properties::new().read().notify()),
        )?;
        let power_switch_handle = power_switch.build();

        let _service_handle = service_builder.build();
        
        Ok(DataService {
            rx: power_switch_handle.value_handle,
            tx: power_switch_handle.value_handle,
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

