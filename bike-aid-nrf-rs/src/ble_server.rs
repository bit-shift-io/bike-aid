use crate::ble_service_data::{self, DataService};
use crate::ble_service_device:: DeviceInformationService;
use crate::ble_service_battery::BatteryService;
use crate::ble_service_settings::{self, SettingsService};
use crate::ble_service_uart::{self, UARTService};
use futures::future::join3;
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

    fn on_write(&self, conn: &Connection, handle: u16, _op: WriteOp, _offset: usize, data: &[u8]) -> Option<Self::Event> {
        self.battery.on_write(handle, data);
        self.settings.on_write(conn, handle, data);
        self.uart.on_write(handle, data);
        None
    }
    
    fn on_deferred_read(&self, handle: u16, offset: usize, reply: nrf_softdevice::ble::DeferredReadReply) -> Option<Self::Event> {
        let _ = (handle, offset, reply);
        panic!("on_deferred_read needs to be implemented for this gatt server");
    }
    
    fn on_deferred_write(
        &self,
        handle: u16,
        op: WriteOp,
        offset: usize,
        data: &[u8],
        reply: nrf_softdevice::ble::DeferredWriteReply,
    ) -> Option<Self::Event> {
        let _ = (handle, op, offset, data, reply);
        panic!("on_deferred_write needs to be implemented for this gatt server");
    }
    
    /// Callback to indicate that one or more characteristic notifications have been transmitted.
    fn on_notify_tx_complete(&self, conn: &Connection, count: u8) -> Option<Self::Event> {
        let _ = (conn, count);
        None
    }
    
    /// Callback to indicate that the services changed indication has been received by the client.
    fn on_services_changed_confirm(&self, conn: &Connection) -> Option<Self::Event> {
        let _ = conn;
        None
    }
    
    fn on_timeout(&self, conn: &Connection) -> Option<Self::Event> {
        let _ = conn;
        None
    }

    /// Callback to indicate that the indication of a characteristic has been received by the client.
    fn on_indicate_confirm(&self, conn: &Connection, handle: u16) -> Option<Self::Event> {
        let _ = (conn, handle);
        None
    }

}


pub async fn run(connection: &Connection, server: &Server) {
    // TODO: add services here
    let data_future = ble_service_data::run(connection, server);
    let settings_future = ble_service_settings::run(connection, server);
    let uart_future = ble_service_uart::run(connection, server);
    //pin_mut!(data_future, settings_future, uart_future);
    join3(data_future, settings_future, uart_future).await;
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