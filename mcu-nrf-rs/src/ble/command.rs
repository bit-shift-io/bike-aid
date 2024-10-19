use core::{cmp::Ordering, fmt};
use crate::utils::globals;

pub struct BleCommand {
    pub priority: u8,
    pub handle: BleHandles,
    pub data: [u8; globals::BLE_BUFFER_LENGTH],
    pub data_len: usize,
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
}


impl Eq for BleCommand {}


impl PartialEq for BleCommand {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}


impl Ord for BleCommand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority)
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
        };
        write!(f, "{}", name)
    }
}


// so we can print the whole command struct!
impl fmt::Debug for BleCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BleCommand")
            .field("priority", &self.priority)
            .field("handle", &self.handle) // This will now use the custom Debug implementation
            .field("data_len", &self.data_len)
            .finish()
    }
}
