
use crate::ble_information_service::{DeviceInformation, DeviceInformationService, PnPID, VidSource};
use crate::ble_battery_service::BatteryService;
use crate::ble_settings_service::SettingsService;

use nrf_softdevice::ble::gatt_server::{RegisterError, WriteOp};
use nrf_softdevice::ble::{gatt_server, Connection};
use nrf_softdevice::Softdevice;


const SERIAL_NUMBER_STRING: &str = "0000";
const MODEL_NUMBER_STRING: &str = "0001";
const MANUFACTURER_NAME_STRING: &str = "Bronson Mathews";
const EMAIL_STRING: &str = "bronsonmathews@gmail.com";


// server
pub struct Server {
    _dis: DeviceInformationService,
    bas: BatteryService,
    settings: SettingsService,
}

impl Server {
    pub fn new(sd: &mut Softdevice) -> Result<Self, RegisterError> {
        // TODO: move device info to its file
        let dis = DeviceInformationService::new(
            sd,
            &PnPID {
                vid_source: VidSource::UsbIF,
                vendor_id: 0xDEAD,
                product_id: 0xBEEF,
                product_version: 0x0000,
            },
            DeviceInformation {
                manufacturer_name: Some(MANUFACTURER_NAME_STRING),
                model_number: Some(MODEL_NUMBER_STRING),
                serial_number: Some(SERIAL_NUMBER_STRING),
                email: Some(EMAIL_STRING),
                ..Default::default()
            },
        )?;

        let bas = BatteryService::new(sd)?;

        let settings = SettingsService::new(sd)?;

        Ok(Self { _dis: dis, bas, settings })
    }
}

impl gatt_server::Server for Server {
    type Event = ();

    fn on_write(
        &self,
        _conn: &Connection,
        handle: u16,
        _op: WriteOp,
        _offset: usize,
        data: &[u8],
    ) -> Option<Self::Event> {
        self.bas.on_write(handle, data);
        self.settings.on_write(handle, data);
        None
    }
}

