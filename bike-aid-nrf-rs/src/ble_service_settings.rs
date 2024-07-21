use defmt::info;
use nrf_softdevice::ble::gatt_server::builder::ServiceBuilder;
use nrf_softdevice::ble::gatt_server::characteristic::{Attribute, Metadata, Properties};
use nrf_softdevice::ble::gatt_server::{self, RegisterError};
use nrf_softdevice::ble::{Connection, Uuid};
use nrf_softdevice::Softdevice;

use crate::ble_server::Server;
use crate::signals;


// TODO: proper uids?
const SERVICE_ID: Uuid = Uuid::new_16(0x1000);
const POWER_SWITCH: Uuid = Uuid::new_16(0x1001);
const LIGHT_SWITCH: Uuid = Uuid::new_16(0x1002);
const HORN_SWITCH: Uuid = Uuid::new_16(0x1003);
const ALARM_ENABLED: Uuid = Uuid::new_16(0x1004);
const THROTTLE_SMOOTHING: Uuid = Uuid::new_16(0x1005);

// TODO: all user modified settings here
pub struct SettingsService {
    power_switch: u16,
    light_switch: u16,
    horn_switch: u16,
    alarm_enabled: u16,
    throttle_smoothing: u16,
}

impl SettingsService {
    pub fn new(sd: &mut Softdevice) -> Result<Self, RegisterError> {
        let mut service_builder = ServiceBuilder::new(sd, SERVICE_ID)?;

        let true_u8 = true as u8;
        let false_u8 = false as u8;

        let power_switch = service_builder.add_characteristic(
            POWER_SWITCH,
            Attribute::new([false_u8]),
            Metadata::new(Properties::new().read().write()),
        )?;
        let power_switch_handle = power_switch.build();

        let light_switch = service_builder.add_characteristic(
            LIGHT_SWITCH,
            Attribute::new([false_u8]),
            Metadata::new(Properties::new().read().write()),
        )?;
        let light_switch_handle = light_switch.build();

        let horn_switch = service_builder.add_characteristic(
            HORN_SWITCH,
            Attribute::new([false_u8]),
            Metadata::new(Properties::new().read().write()),
        )?;
        let horn_switch_handle = horn_switch.build();

        let alarm_enabled = service_builder.add_characteristic(
            ALARM_ENABLED,
            Attribute::new([false_u8]),
            Metadata::new(Properties::new().read().write()),
        )?;
        let alarm_enabled_handle = alarm_enabled.build();

        let throttle_smoothing = service_builder.add_characteristic(
            THROTTLE_SMOOTHING,
            Attribute::new([255u8]),
            Metadata::new(Properties::new().read().write()),
        )?;
        let throttle_smoothing_handle = throttle_smoothing.build();

        let _service_handle = service_builder.build();
        
        Ok(SettingsService {
            power_switch: power_switch_handle.value_handle,
            light_switch: light_switch_handle.value_handle,
            horn_switch: horn_switch_handle.value_handle,
            alarm_enabled: alarm_enabled_handle.value_handle,
            throttle_smoothing: throttle_smoothing_handle.value_handle,
        })
    }

    pub fn on_write(&self, conn: &Connection, handle: u16, data: &[u8]) {
        if data.is_empty() {
            return;
        }

        if handle == self.alarm_enabled {
            
            info!("alarm enabled: {:?}", data);
        }

        if handle == self.throttle_smoothing {
            info!("throttle smoothing: {:?}", data);
        }

        if handle == self.power_switch {
            let message = if data[0] == 1 { true } else { false };
            //gatt_server::notify_value(conn, self.input_keyboard, val).unwrap(); // todo: need to notify server?
            signals::SWITCH_POWER.dyn_immediate_publisher().publish_immediate(message);
            // TODO: do i need to set self power switch here?
            //self.power_switch = 
        }   

        if handle == self.light_switch {
            info!("light switch: {:?}", data);
        }

        if handle == self.horn_switch {
            info!("horn switch: {:?}", data);
        }   

    }
}

pub async fn run(connection: &Connection, server: &Server) {
    info!("run settings service");

}