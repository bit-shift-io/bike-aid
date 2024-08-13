use super::service_data::{self, DataService};
use super::service_device::DeviceInformationService;
use super::service_battery::{self, BatteryService};
use super::service_settings::{self, SettingsService};
use super::service_uart::{self, UARTService};
use defmt::{info, unwrap};
use embassy_futures::join;
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
            data,
            uart,
        })
    }
}

impl gatt_server::Server for Server {
    type Event = ();

    fn on_write(&self, connection: &Connection, handle: u16, _op: WriteOp, _offset: usize, data: &[u8]) -> Option<Self::Event> {
        self.battery.on_write(connection, handle, data);
        self.settings.on_write(connection, handle, data);
        self.data.on_write(connection, handle, data);
        self.uart.on_write(connection, handle, data);
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
        //info!("on_notify_tx_complete: {}", count);
        let _ = (conn, count);
        None
    }
    
    /// Callback to indicate that the services changed indication has been received by the client.
    fn on_services_changed_confirm(&self, conn: &Connection) -> Option<Self::Event> {
        info!("on_services_changed_confirm");
        let _ = conn;
        None
    }
    
    fn on_timeout(&self, conn: &Connection) -> Option<Self::Event> {
        let _ = conn;
        None
    }

    /// Callback to indicate that the indication of a characteristic has been received by the client.
    fn on_indicate_confirm(&self, conn: &Connection, handle: u16) -> Option<Self::Event> {
        info!("on_indicate_confirm",);
        let _ = (conn, handle);
        None
    }

}


pub async fn run(connection: &Connection, server: &Server) {
    info!("BLUETOOTH: device connected");
    // TODO: add services here
    // do we need to mutpin? pin_mut!(...);
    join::join4(
        service_data::run(connection, server), 
        service_settings::run(connection, server),
        service_uart::run(connection, server),
        service_battery::run(connection, server)
        ).await;
}


// shortcut to gatt_server::notify_value
pub fn notify_value(conn: &Connection, handle: u16, val: &[u8]) -> Result<(), NotifyValueError> {
    //gatt_server::notify_value(conn, handle, val) // old direct call

    
    // try notify, if fails, set
    let result = gatt_server::notify_value(conn, handle, val);
    match result { // notify
        Ok(_) => (),
        Err(_) => unwrap!(set_value(handle, val)), // else set
    };
    result // return notify result
     
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