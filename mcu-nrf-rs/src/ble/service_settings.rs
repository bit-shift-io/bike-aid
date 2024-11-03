use nrf_softdevice::ble::gatt_server::builder::ServiceBuilder;
use nrf_softdevice::ble::gatt_server::characteristic::{Attribute, Metadata, Properties};
use nrf_softdevice::ble::gatt_server::{CharacteristicHandles, RegisterError};
use nrf_softdevice::ble::{Connection, Uuid};
use nrf_softdevice::Softdevice;
use crate::utils::signals;


const UUID_SETTINGS_SERVICE: Uuid = Uuid::new_16(0x1000);
const UUID_POWER_ON: Uuid = Uuid::new_16(0x1001);
//const LIGHT_SWITCH: Uuid = Uuid::new_16(0x1002);
//const HORN_SWITCH: Uuid = Uuid::new_16(0x1003);
const UUID_ALARM_ON: Uuid = Uuid::new_16(0x1004);


pub struct SettingsService {
    pub power_on: CharacteristicHandles,
    //light_switch: CharacteristicHandles,
    //horn_switch: CharacteristicHandles,
    pub alarm_on: CharacteristicHandles,
}


impl SettingsService {
    pub fn new(sd: &mut Softdevice) -> Result<Self, RegisterError> {
        let mut service_builder = ServiceBuilder::new(sd, UUID_SETTINGS_SERVICE)?;

        let characteristic_builder = service_builder.add_characteristic(
            UUID_POWER_ON,
            Attribute::new(&[0u8]),
            Metadata::new(Properties::new().read().write().notify()),
        )?;
        let power_on_handle = characteristic_builder.build();

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
            UUID_ALARM_ON,
            Attribute::new(&[0u8]),
            Metadata::new(Properties::new().read().write().notify()),
        )?;
        let alarm_on_handle = characteristic_builder.build();

        let _service_handle = service_builder.build();
        
        Ok(SettingsService {
            power_on: power_on_handle,
            //light_switch: light_switch_handle,
            //horn_switch: horn_switch_handle,
            alarm_on: alarm_on_handle,
        })
    }

    pub fn on_write(&self, _conn: &Connection, handle: u16, data: &[u8]) {
        if handle == self.alarm_on.value_handle {
            let message = if data[0] == 205 { true } else { false };
            signals::ALARM_ENABLED.dyn_sender().send_if_modified(|value| {
                if *value != Some(message) {
                    *value = Some(message);
                    true
                } else { false } // no change
            });
        }

        if handle == self.power_on.value_handle {
            let message = if data[0] == 183 { true } else { false };
            //info!("ble power on: {:?} {:?}", message, data[0]);
            signals::POWER_ON.dyn_sender().send_if_modified(|value| {
                if *value != Some(message) {
                    *value = Some(message);
                    true
                } else { false } // no change
            });
        }   

        // if handle == self.light_switch.value_handle {
        //     info!("light switch: {:?}", data);
        // }

        // if handle == self.horn_switch.value_handle {
        //     info!("horn switch: {:?}", data);
        // }   
    }
}