use crate::utils::{globals, signals};
use super::command::{BleCommand, BleHandles};
use super::service_data::DataService;
use super::service_device::DeviceInformationService;
use super::service_battery::BatteryService;
use super::service_settings::SettingsService;
use super::service_uart::UartService;
use defmt::info;
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::signal::Signal;
use embassy_sync::watch::Watch;
use embassy_time::{Instant, Timer};
use nrf_softdevice::ble::gatt_server::{NotifyValueError, RegisterError, SetValueError, WriteOp};
use nrf_softdevice::ble::{gatt_server, Connection};
use nrf_softdevice::{RawError, Softdevice};
use nrf_softdevice::raw;
use embassy_sync::priority_channel::{PriorityChannel, Min};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, mutex::Mutex};

// Min is smaller numbers as priority
// N is number of messages available
pub static QUEUE_CHANNEL: PriorityChannel::<CriticalSectionRawMutex, BleCommand, Min, 16> = PriorityChannel::new(); 
//pub static NOTIFY_COMPLETE_SIGNAL: Signal<CriticalSectionRawMutex, bool> = Signal::new();

const TASK_ID: &str = "BLUETOOTH";

pub struct Server {
    pub settings: SettingsService,
    pub battery: BatteryService,
    pub data: DataService,
    pub uart: UartService,
    pub _device_informaton: DeviceInformationService,
}


impl Server {
    pub fn new(sd: &mut Softdevice) -> Result<Self, RegisterError> {
        let settings = SettingsService::new(sd)?;
        let battery = BatteryService::new(sd)?;
        let data = DataService::new(sd)?;
        let uart = UartService::new(sd)?;
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
        info!("on_write");
        self.battery.on_write(connection, handle, data);
        self.settings.on_write(connection, handle, data);
        self.uart.on_write(connection, handle, data);
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
        //let send_notify_complete = NOTIFY_COMPLETE_WATCH.sender();
        //send_notify_complete.send(true);

        info!("on_notify_tx_complete");
        NOTIFY_COMPLETE_SIGNAL.signal(true);

        // // if this causes issues, use a watch?
        //let notify_complete = NOTIFY_COMPLETE.lock(); // block the current thread until the mutex is locked
        //*notify_complete = true; // Signal that notification is complete

        
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
    let send_piezo = signals::PIEZO_MODE_WATCH.sender();
    send_piezo.send(signals::PiezoModeType::Notify);
}


pub async fn disconnected(_connection: &Connection, _server: &Server) {
    info!("{}: device disconnected", TASK_ID);
    let send_piezo = signals::PIEZO_MODE_WATCH.sender();
    send_piezo.send(signals::PiezoModeType::Notify);
}


pub async fn run(connection: &Connection, server: &Server) {
    connected(connection, server).await;

    // command queue
    // TODO: inspect the queue, see why when we begin we have 8 items. Are we reciving some dud data?
    let rec_queue = QUEUE_CHANNEL.receiver();
    let send_led = signals::LED_DEBUG_MODE_WATCH.sender();
    //rec_queue.clear(); // clear the channel
    //let mut rec_notify_complete = NOTIFY_COMPLETE_WATCH.receiver().unwrap();
    //let send_notify_complete = NOTIFY_COMPLETE_WATCH.sender();
    //send_notify_complete.send(true); // clear initial value

    NOTIFY_COMPLETE_SIGNAL.signal(true);
    
    loop {
        // i dont think this is needed!
        /*
        info!("wait lock ....");
        // this is my command queue. We need to wait until the previous notification is complete
        // this is not working correctly
        let signal = NOTIFY_COMPLETE_SIGNAL.wait().await;
        info!("send value | signal {}", signal);
    
        // this is now an issue with the android app
        // so for now we leave this here
        Timer::after_millis(150).await;
*/

        //info!("queue: {}", rec_queue.len());

        // wait for command
        let command = rec_queue.receive().await;

        let value: &[u8] = &command.data[..command.data_len];
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
            signals::BleHandles::Uart => handle = server.uart.tx.value_handle,
        }

        info!("{}", command);
        send_led.send(signals::LedModeType::Instant);

        let result = notify_value(connection, handle, value);
        match result { // notify
            Ok(_) => {
                info!("notify_value ok");
            },
            Err(_) => {
                info!("notify_value error, try set_value");
                let set_result = set_value(handle, value);
                match set_result {
                    Ok(_) => {
                        info!("set_value ok");
                    },
                    Err(_) => {
                        info!("set_value error");
                    }
                }
            }
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


pub async fn send_queue(handle: BleHandles, data: &[u8]) {
    Timer::after_ticks(1).await; // allow at least 1 tick between queing items

    let send_ble_queue = QUEUE_CHANNEL.sender();

    let data_len = data.len();
    if data_len > globals::BLE_BUFFER_LENGTH {
        panic!("Data length exceeds buffer size");
    }

    let mut buffer = [0u8; globals::BLE_BUFFER_LENGTH];
    buffer[..data_len].copy_from_slice(data);

    let time = Instant::now();

    // Check if the handle should be a single instance
    // if handle.is_single_instance() {
    //     info!("delete old item {}", handle);
    //     // Lock the channel and modify the queue
    //     QUEUE_CHANNEL.try_receive()
    //     //let mut queue = ble_queue..lock().await; // Adjust as per your concurrency model
    //     //queue.retain(|command| command.handle != handle);
    // }

    let _ = send_ble_queue.try_send(BleCommand {
        time,
        handle,
        data: buffer,
        data_len,
    });
}
