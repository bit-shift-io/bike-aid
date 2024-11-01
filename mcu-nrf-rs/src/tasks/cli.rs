use crate::{piezo::PiezoMode, utils::{globals, signals}};
use defmt::info;
use embassy_time::{Instant, Timer};
use heapless::String;
use core::fmt::Write;

const TASK_ID: &str = "CLI";

#[embassy_executor::task]
pub async fn task() {
    info!("{}", TASK_ID);
    let start_time = Instant::now();
    let mut rec_read = signals::WATCH_CLI.receiver().unwrap();
 
    loop {
        let data = rec_read.changed().await;
        let string = data.as_string();
        let mut result = false;
        
        //info!("{}: {}", TASK_ID, string.as_str());

        if string.contains("panic") && string.contains("test")  {
            defmt::panic!("cli test panic");
        }

        if string.contains("crash") || string.contains("report") || string.contains("panic") {
            if let Some(msg) = panic_persist::get_panic_message_bytes() {
                send(msg).await;
            } else {
                send(b"no panic message").await;
            }
            result = true;    
        }

        if string.contains("uptime") {
            let seconds = start_time.elapsed().as_secs();
            let days = seconds / 86400; // 86400 seconds in a day
            let hours = (seconds % 86400) / 3600; // 3600 seconds in an hour
            let minutes = (seconds % 3600) / 60; // 60 seconds in a minute
            let mut str: String<{globals::BUFFER_LENGTH}> = String::new();
            let _ = write!(str,"{} days {} hours {} minutes", days, hours, minutes);
            send(str.as_bytes()).await;
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

        if string.starts_with("settings") {
            if string.ends_with("write") {
               // TODO
            }
            result = true;
        }

        if string.starts_with("reboot") || string.starts_with("restart") || string.starts_with("reset") {
            send(b"reset in 2 seconds...").await;
            Timer::after_secs(2).await;
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
            send(b"1. passthrough 0/1").await;
            send(b"2. increase_smooth_factor int").await;
            send(b"3. decrease_smooth_factor int").await;
            send(b"4. no_throttle int - mv").await;
            send(b"5. full_throttle int - mv").await;
            send(b"6. deadband_min int - mv").await;
            send(b"7. deadband_max int - mv").await;
            send(b"8. speed_limit int - kmhr").await;
            result = true;
        }
        
        // publish
        if result {
            send(b"ok").await;
        } else {
            send(b"error").await;
        }
    }
}


pub async fn send(data: &[u8]) {
    info!("{}", core::str::from_utf8(&data[..data.len()]).unwrap());
    signals::send_ble(signals::BleHandles::Uart, data).await;
}