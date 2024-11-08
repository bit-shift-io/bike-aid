use crate::utils::{globals, signals};
use super::command::{BleCommand, BleHandles};
use super::service_data::DataService;
use super::service_device::DeviceInformationService;
use super::service_battery::BatteryService;
use super::service_fast_pair::FastPairService;
use super::service_settings::SettingsService;
use super::service_uart::UartService;
use defmt::{info, warn};
use embassy_time::Timer;
use nrf_softdevice::ble::gatt_server::{NotifyValueError, RegisterError, SetValueError, WriteOp};
use nrf_softdevice::ble::{gatt_server, Connection};
use nrf_softdevice::{RawError, Softdevice};
use nrf_softdevice::raw;
use embassy_sync::priority_channel::{PriorityChannel, Min};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;

// Min is smaller numbers as priority
// N is number of messages available
pub static QUEUE_CHANNEL: PriorityChannel::<CriticalSectionRawMutex, BleCommand, Min, 16> = PriorityChannel::new(); 

const TASK_ID: &str = "BLUETOOTH";

pub struct Server {
    pub settings: SettingsService,
    pub battery: BatteryService,
    pub data: DataService,
    pub uart: UartService,
    pub fast_pair: FastPairService,
    pub _device_informaton: DeviceInformationService,
}


impl Server {
    pub fn new(sd: &mut Softdevice) -> Result<Self, RegisterError> {
        let settings = SettingsService::new(sd)?;
        let battery = BatteryService::new(sd)?;
        let data = DataService::new(sd)?;
        let uart = UartService::new(sd)?;
        let fast_pair = FastPairService::new(sd)?;
        let device_informaton = DeviceInformationService::new(sd)?;

        Ok(Self {
            settings,
            battery,   
            data,
            uart,
            fast_pair,
            _device_informaton: device_informaton,
        })
    }
}


impl gatt_server::Server for Server {
    type Event = ();

    fn on_write(&self, connection: &Connection, handle: u16, _op: WriteOp, _offset: usize, data: &[u8]) -> Option<Self::Event> {
        //info!("on_write");
        self.battery.on_write(connection, handle, data);
        self.settings.on_write(connection, handle, data);
        self.uart.on_write(connection, handle, data);
        self.fast_pair.on_write(connection, handle, data);
        None
    }
    
    fn on_deferred_read(&self, handle: u16, offset: usize, reply: nrf_softdevice::ble::DeferredReadReply) -> Option<Self::Event> {
        let _ = (handle, offset, reply);
        panic!("on_deferred_read needs to be implemented for this gatt server");
    }
    
    fn on_deferred_write(&self, handle: u16, op: WriteOp, offset: usize, data: &[u8], reply: nrf_softdevice::ble::DeferredWriteReply) -> Option<Self::Event> {
        let _ = (handle, op, offset, data, reply);
        panic!("on_deferred_write needs to be implemented for this gatt server");
    }
    
    /// Callback to indicate that one or more characteristic notifications have been transmitted.
    fn on_notify_tx_complete(&self, conn: &Connection, count: u8) -> Option<Self::Event> {
        // TODO: unlock here if needed
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


pub async fn connected(_connection: &Connection, _server: &Server) {
    info!("{}: device connected", TASK_ID);
    let send_piezo = signals::PIEZO_MODE.sender();
    send_piezo.send(signals::PiezoModeType::Notify);
}


pub async fn disconnected(_connection: &Connection, _server: &Server) {
    info!("{}: device disconnected", TASK_ID);
    let send_piezo = signals::PIEZO_MODE.sender();
    send_piezo.send(signals::PiezoModeType::Notify);
}


pub async fn run(connection: &Connection, server: &Server) {
    connected(connection, server).await;

    // command queue
    let rec_queue = QUEUE_CHANNEL.receiver();
    let send_led = signals::LED_DEBUG_MODE.sender();
  
    loop {
        // TODO: lock here if needed, but we don't need it
        //info!("queue: {}", rec_queue.len());

        // wait for command
        let command = rec_queue.receive().await;
        if rec_queue.is_full() { warn!("{}: queue full", TASK_ID); }

        let value: &[u8] = command.as_bytes();
        let handle;
        match command.handle {
            BleHandles::BatteryLevel => handle = server.battery.level.value_handle,
            BleHandles::BatteryPower => handle = server.battery.power.value_handle,
            BleHandles::Speed => handle = server.data.speed.value_handle,
            BleHandles::Odometer => handle = server.data.odometer.value_handle,
            BleHandles::Temperature => handle = server.data.temperature.value_handle,
            BleHandles::ClockMinutes => handle = server.data.clock_minutes.value_handle,
            BleHandles::ClockHours => handle = server.data.clock_hours.value_handle,
            BleHandles::BrakeOn => handle = server.data.brake_on.value_handle,
            BleHandles::ParkBrakeOn => handle = server.data.park_brake_on.value_handle,
            BleHandles::CruiseLevel => handle = server.data.cruise_level.value_handle,
            BleHandles::PowerOn => handle = server.settings.power_on.value_handle,
            BleHandles::AlarmOn => handle = server.settings.alarm_on.value_handle,
            BleHandles::Uart => handle = server.uart.tx.value_handle,
            BleHandles::ThrottleLevel => handle = server.data.throttle_level.value_handle,
        }

        //info!("{}", command);
        send_led.send(signals::LedModeType::Instant);

        // first we set the value
        let set_result = set_value(handle, value);
        match set_result {
            Ok(_) => {},
            Err(_) => { info!("{}: set error {}", TASK_ID, command.handle); }
        }

        // then we notify
        let notify_result = notify_value(connection, handle, value);
        match notify_result {
            Ok(_) => {},
            Err(_) => { info!("{}: notify error {}", TASK_ID, command.handle); } // notify not yet enabled usually
        }
    }
}


pub fn notify_value(conn: &Connection, handle: u16, val: &[u8]) -> Result<(), NotifyValueError> {
    gatt_server::notify_value(conn, handle, val)
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


pub fn send_queue(handle: BleHandles, data: &[u8]) {
    let send_ble_queue = QUEUE_CHANNEL.sender();
   
    // If the data is larger than the buffer length, split it into chunks
    if data.len() > globals::BUFFER_LENGTH {
        let mut chunks = data.chunks(globals::BUFFER_LENGTH);
        while let Some(chunk) = chunks.next() {
            embassy_time::block_for(embassy_time::Duration::from_ticks(1));
            let msg = BleCommand::new(handle, chunk);
            let _ = send_ble_queue.try_send(msg);
        }
    } else {
        embassy_time::block_for(embassy_time::Duration::from_ticks(1));
        let msg = BleCommand::new(handle, data);
        let _ = send_ble_queue.try_send(msg);
    }
}