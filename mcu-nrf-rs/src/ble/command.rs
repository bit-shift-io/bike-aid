use core::{cmp::Ordering, fmt};
use defmt::warn;
use embassy_time::Instant;
use crate::utils::globals;


pub struct BleCommand {
    pub time: Instant,
    pub handle: BleHandles,
    pub data: [u8; globals::BUFFER_LENGTH],
    pub data_len: usize,
}

impl BleCommand {
    pub fn new(handle: BleHandles, data: &[u8]) -> BleCommand {
        let mut data_len = data.len();
        if data_len > globals::BUFFER_LENGTH {
            warn!("Data length exceeds buffer size, trimming to {}", globals::BUFFER_LENGTH);
            data_len = globals::BUFFER_LENGTH;
        }

        let mut buffer = [0u8; globals::BUFFER_LENGTH];
        buffer[..data_len].copy_from_slice(&data[..data_len]);

        BleCommand {
            time: Instant::now(),
            handle,
            data: buffer,
            data_len,
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.data[..self.data_len]
    }
}

// order important! this is used for priority
#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, defmt::Format)]
pub enum BleHandles {
    PowerOn,
    AlarmOn,
    Uart,
    BrakeOn,
    ParkBrakeOn,
    CruiseLevel,
    ClockMinutes,
    ClockHours,
    BatteryPower,
    BatteryLevel,
    Speed,
    Odometer,
    Temperature,
    ThrottleLevel,
}


impl BleHandles {
    pub fn is_single_instance(&self) -> bool {
        match self {
            BleHandles::Uart => false, // Example: Uart can have multiple instances
            _ => true, // All other handles are single instance by default
        }
    }
}


impl Eq for BleCommand {}


impl PartialEq for BleCommand {
    fn eq(&self, other: &Self) -> bool {
        self.handle == other.handle && self.time == other.time
    }
}


impl Ord for BleCommand {
    fn cmp(&self, other: &Self) -> Ordering {
        // First compare by handle, then by time
        match self.handle.cmp(&other.handle) {
            Ordering::Equal => self.time.cmp(&other.time),
            other => other,
        }
    }
}


impl PartialOrd for BleCommand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


// Implement Debug for BleHandles
impl fmt::Debug for BleHandles {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            BleHandles::PowerOn => "PowerOn",
            BleHandles::AlarmOn => "AlarmOn",
            BleHandles::Uart => "Uart",
            BleHandles::BrakeOn => "BrakeOn",
            BleHandles::ParkBrakeOn => "ParkBrakeOn",
            BleHandles::CruiseLevel => "CruiseLevel",
            BleHandles::ClockMinutes => "ClockMinutes",
            BleHandles::ClockHours => "ClockHours",
            BleHandles::BatteryPower => "BatteryPower",
            BleHandles::BatteryLevel => "BatteryLevel",
            BleHandles::Speed => "Speed",
            BleHandles::Odometer => "Odometer",
            BleHandles::Temperature => "Temperature",
            BleHandles::ThrottleLevel => "ThrottleLevel",
        };
        write!(f, "{}", name)
    }
}


impl defmt::Format for BleCommand {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "BleCommand {{ time: {}, handle: {}, data: {:?}, data_len: {} }}",
                      self.time, self.handle, self.as_bytes(), self.data_len);
    }
}

// so we can print the whole command struct!
impl fmt::Debug for BleCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BleCommand")
            .field("time", &self.time)
            .field("handle", &self.handle) // This will now use the custom Debug implementation
            .field("data", &self.as_bytes())
            .field("data_len", &self.data_len)
            .finish()
    }
}
