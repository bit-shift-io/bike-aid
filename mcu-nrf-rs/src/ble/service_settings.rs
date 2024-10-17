use embassy_time::Timer;
use nrf_softdevice::ble::gatt_server::builder::ServiceBuilder;
use nrf_softdevice::ble::gatt_server::characteristic::{Attribute, Metadata, Properties};
use nrf_softdevice::ble::gatt_server::{CharacteristicHandles, RegisterError};
use nrf_softdevice::ble::{Connection, Uuid};
use nrf_softdevice::Softdevice;
use embassy_futures::join;
use super::server::{self, *};
use crate::utils::signals;


// TODO: proper uids?
const SERVICE_ID: Uuid = Uuid::new_16(0x1000);
const POWER_ON: Uuid = Uuid::new_16(0x1001);
//const LIGHT_SWITCH: Uuid = Uuid::new_16(0x1002);
//const HORN_SWITCH: Uuid = Uuid::new_16(0x1003);
const ALARM_ON: Uuid = Uuid::new_16(0x1004);
//const THROTTLE_SMOOTHING: Uuid = Uuid::new_16(0x1005);


// TODO: all user modified settings here
pub struct SettingsService {
    pub power_on: CharacteristicHandles,
    //light_switch: CharacteristicHandles,
    //horn_switch: CharacteristicHandles,
    pub alarm_on: CharacteristicHandles,
    //throttle_smoothing: CharacteristicHandles,
}

impl SettingsService {
    pub fn new(sd: &mut Softdevice) -> Result<Self, RegisterError> {
        let mut service_builder = ServiceBuilder::new(sd, SERVICE_ID)?;

        let characteristic_builder = service_builder.add_characteristic(
            POWER_ON,
            Attribute::new(&[0u8]),
            Metadata::new(Properties::new().read().write().notify()),
        )?;
        let mut power_on_handle = characteristic_builder.build();
        power_on_handle.value_handle = signals::BleHandles::PowerOn as u16;

        // let characteristic_builder = service_builder.add_characteristic(
        //     LIGHT_SWITCH,
        //     Attribute::new(&[0u8]),
        //     Metadata::new(Properties::new().read().write().notify()),
        // )?;
        // let light_switch_handle = characteristic_builder.build();

        // let characteristic_builder = service_builder.add_characteristic(
        //     HORN_SWITCH,
        //     Attribute::new(&[0u8]),
        //     Metadata::new(Properties::new().read().write().notify()),
        // )?;
        // let horn_switch_handle = characteristic_builder.build();

        let characteristic_builder = service_builder.add_characteristic(
            ALARM_ON,
            Attribute::new(&[0u8]),
            Metadata::new(Properties::new().read().write().notify()),
        )?;
        let mut alarm_on_handle = characteristic_builder.build();
        alarm_on_handle.value_handle = signals::BleHandles::AlarmOn as u16;

        let _service_handle = service_builder.build();
        
        Ok(SettingsService {
            power_on: power_on_handle,
            //light_switch: light_switch_handle,
            //horn_switch: horn_switch_handle,
            alarm_on: alarm_on_handle,
            //throttle_smoothing: throttle_smoothing_handle,
        })
    }

    pub fn on_write(&self, _conn: &Connection, handle: u16, data: &[u8]) {
        if data.is_empty() {
            return;
        }

        if handle == self.alarm_on.value_handle {
            let message = if data[0] == 205 { true } else { false };
            signals::ALARM_ENABLED_WATCH.dyn_sender().send(message);
  
        }

        if handle == self.power_on.value_handle {
            let message = if data[0] == 183 { true } else { false };
            signals::POWER_ON_WATCH.dyn_sender().send(message);
        }   

        // if handle == self.light_switch.value_handle {
        //     info!("light switch: {:?}", data);
        // }

        // if handle == self.horn_switch.value_handle {
        //     info!("horn switch: {:?}", data);
        // }   

    }
}


pub async fn run(connection: &Connection, server: &Server) {
    join::join(
        update_power(connection, server), 
        update_alarm(connection, server), 
        ).await;
}


pub async fn update_power(connection: &Connection, server: &Server) {
    let mut rec = signals::POWER_ON_WATCH.receiver().unwrap();
    let handle = server.settings.power_on.value_handle;
    loop {
        let val = rec.changed().await;
        Timer::after_millis(300).await; // TODO: fix ble to be async? delay to avoid flooding
        let _ = server::notify_value(connection, handle, &[val as u8]);
    }
}


pub async fn update_alarm(connection: &Connection, server: &Server) {
    let mut sub = signals::ALARM_ENABLED_WATCH.receiver().unwrap();
    let handle = server.settings.alarm_on.value_handle;
    loop {
        let val = sub.changed().await;
        Timer::after_millis(100).await; // TODO: fix ble to be async? delay to avoid flooding
        let _ = server::notify_value(connection, handle, &[val as u8]);
    }
}

