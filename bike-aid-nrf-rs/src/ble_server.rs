use crate::ble_service_data::DataService;
use crate::ble_service_device:: DeviceInformationService;
use crate::ble_service_battery::BatteryService;
use crate::ble_service_settings::SettingsService;
use crate::ble_service_uart::UARTService;
use nrf_softdevice::ble::gatt_server::{RegisterError, WriteOp};
use nrf_softdevice::ble::{gatt_server, Connection};
use nrf_softdevice::Softdevice;


pub struct Server {
    pub _device_informaton: DeviceInformationService,
    pub battery: BatteryService,
    pub settings: SettingsService,
    pub _data: DataService,
    pub uart: UARTService,
}

impl Server {
    pub fn new(sd: &mut Softdevice) -> Result<Self, RegisterError> {
        let device_informaton = DeviceInformationService::new(sd)?;
        let battery = BatteryService::new(sd)?;
        let settings = SettingsService::new(sd)?;
        let data = DataService::new(sd)?;
        let uart = UARTService::new(sd)?;

        Ok(Self {
            _device_informaton: device_informaton,
            battery,
            settings,
            _data: data,
            uart,
        })
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
        self.battery.on_write(handle, data);
        self.settings.on_write(handle, data);
        self.uart.on_write(handle, data);
        None
    }
}

