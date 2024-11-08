use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, watch::Watch};
use crate::ble::server;
use super::{data_slice::DataSlice, globals};

// types
//type ChannelMutex = CriticalSectionRawMutex;
type WatchMutex = CriticalSectionRawMutex;

// default values
pub fn init() {
    BRAKE_ON.dyn_sender().send_if_modified(|value| { *value = Some(false); false });
    PARK_BRAKE_ON.dyn_sender().send_if_modified(|value| { *value = Some(true); false });
    CRUISE_LEVEL.dyn_sender().send_if_modified(|value| { *value = Some(0u8); false });
    THROTTLE_IN.dyn_sender().send_if_modified(|value| { *value = Some(0u16); false });
    THROTTLE_OUT.dyn_sender().send_if_modified(|value| { *value = Some(0u16); false });
    REQUEST_POWER_ON.dyn_sender().send_if_modified(|value| { *value = Some(false); false });
    POWER_ON.dyn_sender().send_if_modified(|value| { *value = Some(false); false });
    SWITCH_HORN.dyn_sender().send_if_modified(|value| { *value = Some(false); false });
    SWITCH_LIGHT.dyn_sender().send_if_modified(|value| { *value = Some(false); false });
    INSTANT_SPEED.dyn_sender().send_if_modified(|value| { *value = Some(0u32); false });
    SMOOTH_SPEED.dyn_sender().send_if_modified(|value| { *value = Some(0u8); false });
    WHEEL_ROTATIONS.dyn_sender().send_if_modified(|value| { *value = Some(0u8); false });
    ODOMETER.dyn_sender().send_if_modified(|value| { *value = Some(0u16); false });
    BATTERY_IN.dyn_sender().send_if_modified(|value| { *value = Some([0u16, 0u16]); false });
    
    LED_MODE.dyn_sender().send_if_modified(|value| { *value = Some(LedModeType::None); false });
    LED_DEBUG_MODE.dyn_sender().send_if_modified(|value| { *value = Some(LedModeType::None); false });
    PIEZO_MODE.dyn_sender().send_if_modified(|value| { *value = Some(PiezoModeType::None); false });
    
    ALARM_ENABLED.dyn_sender().send_if_modified(|value| { *value = Some(false); false });
    ALARM_ALERT_ACTIVE.dyn_sender().send_if_modified(|value| { *value = Some(false); false });
    ALARM_MOTION_DETECTED.dyn_sender().send_if_modified(|value| { *value = Some(false); false });
    
    //UART_WRITE.dyn_sender().send_if_modified(|value| { *value = Some(DataSlice {data: [0u8; globals::BLE_BUFFER_LENGTH],data_len: globals::BLE_BUFFER_LENGTH}); false });
    CLI.dyn_sender().send_if_modified(|value| { *value = Some(DataSlice {data: [0u8; globals::BUFFER_LENGTH],data_len: globals::BUFFER_LENGTH}); false });

    STORE_WRITE.dyn_sender().send_if_modified(|value| { *value = Some(false); false });
    STORE_UPDATED.dyn_sender().send_if_modified(|value| { *value = Some(false); false });
}


// == WATCHES ===
// Watches can not have history
// Return None if max receivers is reached
pub static BRAKE_ON: Watch<WatchMutex, bool, 9> = Watch::new();
pub static PARK_BRAKE_ON: Watch<WatchMutex, bool, 5> = Watch::new();
pub static CRUISE_LEVEL: Watch<WatchMutex, u8, 3> = Watch::new();
pub static THROTTLE_IN: Watch<WatchMutex, u16, 3> = Watch::new();
pub static THROTTLE_OUT: Watch<WatchMutex, u16, 1> = Watch::new();
pub static REQUEST_POWER_ON: Watch<WatchMutex, bool, 2> = Watch::new();
pub static POWER_ON: Watch<WatchMutex, bool, 10> = Watch::new();
pub static SWITCH_HORN: Watch<WatchMutex, bool, 1> = Watch::new();
pub static SWITCH_LIGHT: Watch<WatchMutex, bool, 1> = Watch::new();
pub static INSTANT_SPEED: Watch<WatchMutex, u32, 1> = Watch::new();
pub static SMOOTH_SPEED: Watch<WatchMutex, u8, 1> = Watch::new();
pub static WHEEL_ROTATIONS: Watch<WatchMutex, u8, 1> = Watch::new();
pub static ODOMETER: Watch<WatchMutex, u16, 1> = Watch::new();
pub static BATTERY_IN: Watch<WatchMutex, [u16; 2], 1> = Watch::new();

pub type LedModeType = crate::tasks::led::LedMode;
pub static LED_MODE: Watch<WatchMutex, LedModeType, 1> = Watch::new();
pub static LED_DEBUG_MODE: Watch<WatchMutex, LedModeType, 1> = Watch::new();

pub type PiezoModeType = crate::tasks::piezo::PiezoMode;
pub static PIEZO_MODE: Watch<WatchMutex, PiezoModeType, 1> = Watch::new();

pub static ALARM_ENABLED: Watch<WatchMutex, bool, 3> = Watch::new();
pub static ALARM_ALERT_ACTIVE: Watch<WatchMutex, bool, 1> = Watch::new();
pub static ALARM_MOTION_DETECTED: Watch<WatchMutex, bool, 1> = Watch::new();

pub static CLI: Watch<WatchMutex, DataSlice, 1> = Watch::new();

pub static STORE_WRITE: Watch<WatchMutex, bool, 1> = Watch::new();
pub static STORE_UPDATED: Watch<WatchMutex, bool, 1> = Watch::new();


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
