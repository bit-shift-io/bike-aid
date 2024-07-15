use crate::ble_service_data::DataService;
use crate::ble_service_device:: DeviceInformationService;
use crate::ble_service_battery::BatteryService;
use crate::ble_service_settings::SettingsService;
use crate::ble_service_uart::UARTService;
use nrf_softdevice::ble::gatt_server::{NotifyValueError, RegisterError, SetValueError, WriteOp};
use nrf_softdevice::ble::{gatt_server, Connection};
use nrf_softdevice::{RawError, Softdevice};
use nrf_softdevice::raw;

pub struct Server {
    pub _device_informaton: DeviceInformationService,
    pub battery: BatteryService,
    pub settings: SettingsService,
    pub data: DataService,
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
            data: data,
            uart,
        })
    }
}

impl gatt_server::Server for Server {
    type Event = ();

    // notify_value
    fn on_write(
        &self,
        conn: &Connection,
        handle: u16,
        _op: WriteOp,
        _offset: usize,
        data: &[u8],
    ) -> Option<Self::Event> {
        self.battery.on_write(handle, data);
        self.settings.on_write(conn, handle, data);
        self.uart.on_write(handle, data);
        None
    }
}

// shortcut to gatt_server::notify_value
pub fn notify_value(conn: &Connection, handle: u16, val: &[u8]) -> Result<(), NotifyValueError> {
    gatt_server::notify_value(conn, handle, val)
}

// bypass gatt_server::set_value due to it using unused sd reference
pub fn set_value(handle: u16, val: &[u8]) -> Result<(), SetValueError> {
    let mut value = raw::ble_gatts_value_t {
        p_value: val.as_ptr() as _,
        len: val.len() as _,
        offset: 0,
    };
    let ret = unsafe { raw::sd_ble_gatts_value_set(raw::BLE_CONN_HANDLE_INVALID as u16, handle, &mut value) };
    RawError::convert(ret)?;

    Ok(())
}