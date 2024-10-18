use crate::utils::signals;
use super::service_data::DataService;
use super::service_device::DeviceInformationService;
use super::service_battery::BatteryService;
use super::service_settings::SettingsService;
use super::service_uart::UARTService;
use defmt::info;
use nrf_softdevice::ble::gatt_server::{NotifyValueError, RegisterError, SetValueError, WriteOp};
use nrf_softdevice::ble::{gatt_server, Connection};
use nrf_softdevice::{RawError, Softdevice};
use nrf_softdevice::raw;

pub struct Server {
    pub settings: SettingsService,
    pub battery: BatteryService,
    pub data: DataService,
    pub uart: UARTService,
    pub _device_informaton: DeviceInformationService,
}


impl Server {
    pub fn new(sd: &mut Softdevice) -> Result<Self, RegisterError> {
        let settings = SettingsService::new(sd)?;
        let battery = BatteryService::new(sd)?;
        let data = DataService::new(sd)?;
        let uart = UARTService::new(sd)?;
        let device_informaton = DeviceInformationService::new(sd)?;

        Ok(Self {
            settings,
            battery,   
            data,
            uart,
            _device_informaton: device_informaton,
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
        info!("on_timeout");
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
    let send_piezo = signals::PIEZO_MODE_WATCH.sender();
    send_piezo.send(signals::PiezoModeType::Notify);

    // TODO: wait for return value from on_notify_tx_complete before sending next command
    // this will prevent flooding the ble signal resulting in failed sends
    let rec = signals::BLE_QUEUE_CHANNEL.receiver();
    
    loop {
        let command = rec.receive().await;
        let data_slice: &[u8] = &command.data[..command.data_len];
        let handle;
        match command.handle {
            signals::BleHandles::BatteryLevel => handle = server.battery.level.value_handle,
            signals::BleHandles::BatteryPower => handle = server.battery.power.value_handle,
            signals::BleHandles::Speed => handle = server.data.speed.value_handle,
            signals::BleHandles::Odometer => handle = server.data.odometer.value_handle,
            signals::BleHandles::Temperature => handle = server.data.temperature.value_handle,
            signals::BleHandles::ClockMinutes => handle = server.data.clock_minutes.value_handle,
            signals::BleHandles::ClockHours => handle = server.data.clock_hours.value_handle,
            signals::BleHandles::BrakeOn => handle = server.data.brake_on.value_handle,
            signals::BleHandles::ParkBrakeOn => handle = server.data.park_brake_on.value_handle,
            signals::BleHandles::CruiseLevel => handle = server.data.cruise_level.value_handle,
            signals::BleHandles::PowerOn => handle = server.settings.power_on.value_handle,
            signals::BleHandles::AlarmOn => handle = server.settings.alarm_on.value_handle,
            signals::BleHandles::UART => handle = server.uart.tx.value_handle,
        }

        info!("handle: {} -> {} | data: {:?}", command.handle as u16, handle, data_slice);
        let _ = notify_value(connection, handle, data_slice);
    }
}


pub fn notify_value(conn: &Connection, handle: u16, val: &[u8]) -> Result<(), NotifyValueError> {
    // try notify, if fails, set
    
    let result = gatt_server::notify_value(conn, handle, val);
    match result { // notify
        Ok(_) => (),
        Err(_) => {
            info!("notify fail, try set value");
            let result = set_value(handle, val);
            match result { // set result
                Ok(_) => (),
                Err(_) => {
                    info!("set fail");
                    ()
                },
            }
        },
    };
    result // return notify result
}


pub fn set_value(handle: u16, val: &[u8]) -> Result<(), SetValueError> {
    // bypass gatt_server::set_value due to it using unused sd reference
    let mut value = raw::ble_gatts_value_t {
        p_value: val.as_ptr() as _,
        len: val.len() as _,
        offset: 0,
    };
    let ret = unsafe { raw::sd_ble_gatts_value_set(raw::BLE_CONN_HANDLE_INVALID as u16, handle, &mut value) };
    RawError::convert(ret)?;
    Ok(())
}