use crate::{piezo::PiezoMode, utils::{globals, settings, signals}};
use defmt::{info, warn};
use embassy_time::{Instant, Timer};
use heapless::String;
use core::fmt::{Display, Write};

const TASK_ID: &str = "CLI";

#[embassy_executor::task]
pub async fn task() {
    info!("{}", TASK_ID);

    let mut rec_read = signals::CLI.receiver().unwrap();
    let mut rec_throttle_settings = settings::THROTTLE_SETTINGS.receiver().unwrap();
    let send_throttle_settings = settings::THROTTLE_SETTINGS.sender();
 
    loop {
        let data = rec_read.changed().await;
        let string = data.as_string();
        let mut result = false;
        
        //info!("{}: {}", TASK_ID, string.as_str());

        if string.contains("panic") && string.contains("test")  {
            panic!("cli test panic");
        }

        if string.contains("crash") || string.contains("report") || string.contains("panic") {
            if let Some(msg) = panic_persist::get_panic_message_bytes() {
                send(msg);
            } else {
                send(b"no panic message");
            }
            result = true;    
        }

        if string.contains("test")  {
            // place test action here
            warn!("test warning");
            result = true;
        }

        if string.contains("uptime") {
            let seconds = Instant::MIN.elapsed().as_secs();
            let days = seconds / 86400; // 86400 seconds in a day
            let hours = (seconds % 86400) / 3600; // 3600 seconds in an hour
            let minutes = (seconds % 3600) / 60; // 60 seconds in a minute
            let mut str: String<{globals::BUFFER_LENGTH}> = String::new();
            let _ = write!(str,"{} days {} hours {} minutes", days, hours, minutes);
            send(str.as_bytes());
            result = true;
        }

        if string.starts_with("passthrough") || string.starts_with("1") {
            let mut throttle_settings = rec_throttle_settings.try_get().unwrap();
            if string.ends_with("1") {
                throttle_settings.passthrough = true;
            } else {
                throttle_settings.passthrough = false;
            }

            result = true;
        }

        if string.starts_with("increase_smoothing_low") || string.starts_with("2") {
            let mut throttle_settings = rec_throttle_settings.try_get().unwrap();
            let value = string.split_whitespace().last().unwrap();
            throttle_settings.increase_smoothing_low = value.parse::<u16>().unwrap();
            send_throttle_settings.send(throttle_settings);
            result = true;
        }

        if string.starts_with("increase_smoothing_high") || string.starts_with("3") {
            let mut throttle_settings = rec_throttle_settings.try_get().unwrap();
            let value = string.split_whitespace().last().unwrap();
            throttle_settings.increase_smoothing_high = value.parse::<u16>().unwrap();
            send_throttle_settings.send(throttle_settings);
            result = true;
        }

        if string.starts_with("decrease_smooth_factor") || string.starts_with("4") {
            let mut throttle_settings = rec_throttle_settings.try_get().unwrap();
            let value = string.split_whitespace().last().unwrap();
            throttle_settings.decrease_smoothing = value.parse::<u16>().unwrap();
            send_throttle_settings.send(throttle_settings);
            result = true;
        }

        if string.starts_with("no_throttle") || string.starts_with("5") {
            let mut throttle_settings = rec_throttle_settings.try_get().unwrap();
            let value = string.split_whitespace().last().unwrap();
            throttle_settings.throttle_min = value.parse::<u16>().unwrap();
            send_throttle_settings.send(throttle_settings);
            result = true;
        }

        if string.starts_with("full_throttle") || string.starts_with("6") {
            let mut throttle_settings = rec_throttle_settings.try_get().unwrap();
            let value = string.split_whitespace().last().unwrap();
            throttle_settings.throttle_max = value.parse::<u16>().unwrap();
            send_throttle_settings.send(throttle_settings);
            result = true;
        }

        if string.starts_with("deadband_min") || string.starts_with("7") {
            let mut throttle_settings = rec_throttle_settings.try_get().unwrap();
            let value = string.split_whitespace().last().unwrap();
            throttle_settings.deadband_min = value.parse::<u16>().unwrap();
            send_throttle_settings.send(throttle_settings);
            result = true;
        }

        if string.starts_with("deadband_max") || string.starts_with("8") {
            let mut throttle_settings = rec_throttle_settings.try_get().unwrap();
            let value = string.split_whitespace().last().unwrap();
            throttle_settings.deadband_max = value.parse::<u16>().unwrap();
            send_throttle_settings.send(throttle_settings);
            result = true;
        }

        if string.starts_with("speed_step") || string.starts_with("9") {
            let mut throttle_settings = rec_throttle_settings.try_get().unwrap();
            let value = string.split_whitespace().last().unwrap();
            throttle_settings.speed_step = value.parse::<u16>().unwrap();
            send_throttle_settings.send(throttle_settings);
            result = true;
        }


        // if string.starts_with("settings") {
        //     if string.ends_with("write") {
        //        // TODO
        //     }
        //     result = true;
        // }

        if string.starts_with("reboot") || string.starts_with("restart") || string.starts_with("reset") {
            send(b"reset in 2 seconds...");
            Timer::after_secs(2).await;
            cortex_m::peripheral::SCB::sys_reset();
        }

        if string.starts_with("power") {
            if string.ends_with("on") {
                signals::REQUEST_POWER_ON.dyn_sender().send(true);
            } else {
                signals::REQUEST_POWER_ON.dyn_sender().send(false);
            }

            result = true;
        }

        if string.starts_with("alarm") {
            if string.ends_with("on") {
                signals::ALARM_MODE.dyn_sender().send(true);
            } else if string.ends_with("play") {
                signals::ALARM_ALERT_ACTIVE.dyn_sender().send(true);
            } else {
                signals::ALARM_ALERT_ACTIVE.dyn_sender().send(false);
                signals::ALARM_MODE.dyn_sender().send(false);
            }

            result = true;
        }


        if string.starts_with("play") {
            if string.ends_with("tune") {
                signals::PIEZO_MODE.dyn_sender().send(PiezoMode::SuperMarioBros);
            } else if string.ends_with("power") {
                signals::PIEZO_MODE.dyn_sender().send(PiezoMode::PowerOn);
            } else {
                signals::PIEZO_MODE.dyn_sender().send(PiezoMode::SuperMarioBros);
            }

            result = true;
        }


        if string.starts_with("help") || string.starts_with("?") || string == ("h") || string.starts_with("settings") {
            let settings = rec_throttle_settings.try_get().unwrap();

            send(b"1. passthrough");
            if settings.passthrough { send(b"1") } else { send(b"0") };

            send(b"2. increase_smoothing_low");
            send_str(settings.increase_smoothing_low);
    
            send(b"3. increase_smoothing_high");
            send_str(settings.increase_smoothing_high);

            send(b"4. decrease_smooth_factor");
            send_str(settings.decrease_smoothing);

            send(b"5. throttle_min");
            send_str(settings.throttle_min);

            send(b"6. throttle_max");
            send_str(settings.throttle_max);

            send(b"7. deadband_min");
            send_str(settings.deadband_min);

            send(b"8. deadband_max");
            send_str(settings.deadband_max);

            send(b"9. speed_step");
            send_str(settings.speed_step);

            result = true;
        }
        
        // publish
        if result {
            send(b"ok");
        } else {
            send(b"error");
        }
    }
}


pub fn send_str<T: Display>(value: T) {
    let mut str: String<{globals::BUFFER_LENGTH}> = String::new();
    let _ = write!(str, "{}", value);
    send(str.as_bytes());
}

pub fn send(data: &[u8]) {
    info!("{}", core::str::from_utf8(&data[..data.len()]).unwrap());
    signals::send_ble(signals::BleHandles::Uart, data);
}