use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, watch::Watch};
use crate::ble::server;
use super::{data_slice::DataSlice, globals};

// types
//type ChannelMutex = CriticalSectionRawMutex;
type WatchMutex = CriticalSectionRawMutex;


// == WATCHES ===
// Watches can not have history
// Return None if max receivers is reached

pub static ALARM_ENABLED: Watch<WatchMutex, bool, 3> = Watch::new_with(false);
pub static ALARM_ALERT_ACTIVE: Watch<WatchMutex, bool, 1> = Watch::new_with(false);
pub static ALARM_MOTION_DETECTED: Watch<WatchMutex, bool, 1> = Watch::new_with(false);
pub static BRAKE_ON: Watch<WatchMutex, bool, 9> = Watch::new_with(false);
pub static PARK_BRAKE_ON: Watch<WatchMutex, bool, 5> = Watch::new_with(true);
pub static CRUISE_LEVEL: Watch<WatchMutex, u8, 3> = Watch::new_with(0u8);
pub static THROTTLE_IN: Watch<WatchMutex, u16, 3> = Watch::new_with(0u16);
pub static THROTTLE_OUT: Watch<WatchMutex, u16, 1> = Watch::new_with(0u16);
pub static REQUEST_POWER_ON: Watch<WatchMutex, bool, 2> = Watch::new_with(false);
pub static POWER_ON: Watch<WatchMutex, bool, 10> = Watch::new_with(false);
pub static INSTANT_SPEED: Watch<WatchMutex, u32, 1> = Watch::new_with(0u32);
pub static SMOOTH_SPEED: Watch<WatchMutex, u8, 1> = Watch::new_with(0u8);
pub static WHEEL_ROTATIONS: Watch<WatchMutex, u8, 1> = Watch::new_with(0u8);
pub static ODOMETER: Watch<WatchMutex, u16, 1> = Watch::new_with(0u16);
pub static BATTERY_IN: Watch<WatchMutex, [u16; 2], 1> = Watch::new_with([0u16, 0u16]);
pub static CLI: Watch<WatchMutex, DataSlice, 1> = Watch::new_with(DataSlice {data: [0u8; globals::BUFFER_LENGTH],data_len: globals::BUFFER_LENGTH});

pub type LedModeType = crate::tasks::led::LedMode;
pub static LED_MODE: Watch<WatchMutex, LedModeType, 1> = Watch::new_with(LedModeType::None);
pub static LED_DEBUG_MODE: Watch<WatchMutex, LedModeType, 1> = Watch::new_with(LedModeType::None);

pub type PiezoModeType = crate::tasks::piezo::PiezoMode;
pub static PIEZO_MODE: Watch<WatchMutex, PiezoModeType, 1> = Watch::new_with(PiezoModeType::None);


// == CHANNELS ==
// Channels can have history
// <Mutex Type, Data Type, Max Channels(History), Max Subscribers, Max Publishers>
// pub static EXAMPLE: PubSubChannel<ChannelMutex, bool, 1, 2, 2> = PubSubChannel::new();
 

// == FUNCTIONS ==
pub type BleHandles = crate::ble::command::BleHandles;
pub fn send_ble(handle: BleHandles, data: &[u8]) {
    server::send_queue(handle, data);    
}

pub fn send_cli(data: &[u8]) {
    let msg = DataSlice::new(data);
    let sender = CLI.sender();
    let _ = sender.send(msg);
}
