use crate::{piezo::PiezoMode, utils::{globals, signals}};
use defmt::*;
use embassy_time::Instant;
use heapless::String;

const TASK_ID: &str = "CLI";

#[embassy_executor::task]
pub async fn task() {
    info!("{}", TASK_ID);
    let mut rec_read = signals::UART_READ_WATCH.receiver().unwrap();
 
    loop {
        let data = rec_read.changed().await;
        let string = data.as_string();
        let mut result = false;
        
        //info!("{}: {}", TASK_ID, string.as_str());

        if string.contains("uptime") {
            let uptime = Instant::now();
            let seconds = uptime.elapsed().as_secs();
            let days = seconds / 86400; // 86400 seconds in a day
            let hours = (seconds % 86400) / 3600; // 3600 seconds in an hour
            let minutes = (seconds % 3600) / 60; // 60 seconds in a minute
            info!("Uptime: {} days, {} hours, {} minutes", days, hours, minutes);
            let str: String<{globals::BLE_BUFFER_LENGTH}> = String::new();
            // str.push_str(days.to_string());
            // str.push_str(" days ");
            // str.push(hours.to_string());
            // str.push_str(" hours ");
            // str.push(minutes.to_string());
            // str.push_str(" minutes");
            signals::send_ble(signals::BleHandles::Uart, str.as_bytes()).await;
            result = true;
        }

        if string.starts_with("passthrough") || string.starts_with("1") {
            let mut throttle_settings = signals::THROTTLE_SETTINGS_MUTEX.lock().await;
            if string.ends_with("1") {
                throttle_settings.passthrough = true;
            } else {
                throttle_settings.passthrough = false;
            }

            result = true;
        }

        if string.starts_with("increase_smooth_factor") || string.starts_with("2") {
            let mut throttle_settings = signals::THROTTLE_SETTINGS_MUTEX.lock().await;
            let value = string.split_whitespace().last().unwrap();
            throttle_settings.increase_smooth_factor = value.parse::<u16>().unwrap();
            result = true;
        }

        if string.starts_with("decrease_smooth_factor") || string.starts_with("3") {
            let mut throttle_settings = signals::THROTTLE_SETTINGS_MUTEX.lock().await;
            let value = string.split_whitespace().last().unwrap();
            throttle_settings.decrease_smooth_factor = value.parse::<u16>().unwrap();
            result = true;
        }

        if string.starts_with("no_throttle") || string.starts_with("4") {
            let mut throttle_settings = signals::THROTTLE_SETTINGS_MUTEX.lock().await;
            let value = string.split_whitespace().last().unwrap();
            throttle_settings.throttle_min = value.parse::<u16>().unwrap();
            result = true;
        }

        if string.starts_with("full_throttle") || string.starts_with("5") {
            let mut throttle_settings = signals::THROTTLE_SETTINGS_MUTEX.lock().await;
            let value = string.split_whitespace().last().unwrap();
            throttle_settings.throttle_max = value.parse::<u16>().unwrap();
            result = true;
        }

        if string.starts_with("deadband_min") || string.starts_with("6") {
            let mut throttle_settings = signals::THROTTLE_SETTINGS_MUTEX.lock().await;
            let value = string.split_whitespace().last().unwrap();
            throttle_settings.deadband_min = value.parse::<u16>().unwrap();
            result = true;
        }

        if string.starts_with("deadband_max") || string.starts_with("7") {
            let mut throttle_settings = signals::THROTTLE_SETTINGS_MUTEX.lock().await;
            let value = string.split_whitespace().last().unwrap();
            throttle_settings.deadband_max = value.parse::<u16>().unwrap();
            result = true;
        }

        if string.starts_with("speed_limit") || string.starts_with("8") {
            let mut throttle_settings = signals::THROTTLE_SETTINGS_MUTEX.lock().await;
            let value = string.split_whitespace().last().unwrap();
            throttle_settings.speed_limit = value.parse::<u16>().unwrap();
            result = true;
        }


        // TODO
        if string.starts_with("settings") {
            if string.ends_with("write") {
               // TODO
            }
            result = true;
        }

        if string.starts_with("reboot") || string.starts_with("restart") {
            cortex_m::peripheral::SCB::sys_reset();
        }

        if string.starts_with("power") {
            if string.ends_with("on") {
                signals::POWER_ON_WATCH.dyn_sender().send(true);
            } else {
                signals::POWER_ON_WATCH.dyn_sender().send(false);
            }

            result = true;
        }

        if string.starts_with("alarm") {
            if string.ends_with("on") {
                signals::ALARM_ENABLED_WATCH.dyn_sender().send(true);
            } else if string.ends_with("play") {
                signals::ALARM_ALERT_ACTIVE_WATCH.dyn_sender().send(true);
            } else {
                signals::ALARM_ALERT_ACTIVE_WATCH.dyn_sender().send(false);
                signals::ALARM_ENABLED_WATCH.dyn_sender().send(false);
            }

            result = true;
        }


        if string.starts_with("play") {
            if string.ends_with("tune") {
                signals::PIEZO_MODE_WATCH.dyn_sender().send(PiezoMode::SuperMarioBros);
            } else if string.ends_with("power") {
                signals::PIEZO_MODE_WATCH.dyn_sender().send(PiezoMode::PowerOn);
            } else {
                signals::PIEZO_MODE_WATCH.dyn_sender().send(PiezoMode::SuperMarioBros);
            }

            result = true;
        }


        if string.starts_with("help") {
            let str: String<{globals::BLE_BUFFER_LENGTH}> = String::try_from("1. passthrough 0/1").unwrap();
            signals::send_ble(signals::BleHandles::Uart, str.as_bytes()).await;
    
            let str: String<{globals::BLE_BUFFER_LENGTH}> = String::try_from("2. increase_smooth_factor int").unwrap();
            signals::send_ble(signals::BleHandles::Uart, str.as_bytes()).await;

            let str: String<{globals::BLE_BUFFER_LENGTH}> = String::try_from("3. decrease_smooth_factor int").unwrap();
            signals::send_ble(signals::BleHandles::Uart, str.as_bytes()).await;

            let str: String<{globals::BLE_BUFFER_LENGTH}> = String::try_from("4. no_throttle int - mv").unwrap();
            signals::send_ble(signals::BleHandles::Uart, str.as_bytes()).await;

            let str: String<{globals::BLE_BUFFER_LENGTH}> = String::try_from("5. full_throttle int - mv").unwrap();
            signals::send_ble(signals::BleHandles::Uart, str.as_bytes()).await;

            let str: String<{globals::BLE_BUFFER_LENGTH}> = String::try_from("6. deadband_min int - mv").unwrap();
            signals::send_ble(signals::BleHandles::Uart, str.as_bytes()).await;

            let str: String<{globals::BLE_BUFFER_LENGTH}> = String::try_from("7. deadband_max int - mv").unwrap();
            signals::send_ble(signals::BleHandles::Uart, str.as_bytes()).await;

            let str: String<{globals::BLE_BUFFER_LENGTH}> = String::try_from("8. speed_limit int - kmhr").unwrap();
            signals::send_ble(signals::BleHandles::Uart, str.as_bytes()).await;

            result = true;
        }
        
        // publish
        if result {
            let ok: String<{globals::BLE_BUFFER_LENGTH}> = String::try_from("ok").unwrap();
            signals::send_ble(signals::BleHandles::Uart, ok.as_bytes()).await;
        } else {
            let error: String<{globals::BLE_BUFFER_LENGTH}> = String::try_from("error").unwrap();
            signals::send_ble(signals::BleHandles::Uart, error.as_bytes()).await;
        }
    }
}