use core::cmp::Ordering;
use crate::utils::signals;

const BUFFER_SIZE: usize = 32;

pub struct BleCommandQueue {
    pub priority: u8,
    pub handle: QueueHandles,
    pub data: [u8; BUFFER_SIZE],
    pub data_len: usize,
}


#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq)]
pub enum QueueHandles {
    BatteryLevel,
    BatteryPower,
    Speed,
    Odometer,
    Temperature,
    ClockMinutes,
    ClockHours,
    BrakeOn,
    ParkBrakeOn,
    CruiseLevel,
    PowerOn,
    AlarmOn,
    UART,
}


pub async fn submit(priority: u8, handle: QueueHandles, data: &[u8]) {
    let ble_queue = signals::BLE_QUEUE_CHANNEL.sender();

    let data_len = data.len();
    if data_len > BUFFER_SIZE {
        panic!("Data length exceeds buffer size");
    }

    let mut buffer = [0u8; BUFFER_SIZE];
    buffer[..data_len].copy_from_slice(data);

    ble_queue.send(BleCommandQueue {
        priority,
        handle,
        data: buffer,
        data_len,
    }).await;
}


impl Eq for BleCommandQueue {}


impl PartialEq for BleCommandQueue {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}


impl Ord for BleCommandQueue {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority)
    }
}


impl PartialOrd for BleCommandQueue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}