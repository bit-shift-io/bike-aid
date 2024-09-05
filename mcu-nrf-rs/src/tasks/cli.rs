use crate::utils::{signals, store};
use defmt::*;
use embassy_time::Timer;
use heapless::String;

const TASK_ID: &str = "CLI";

#[embassy_executor::task]
pub async fn task() {
    info!("{}: start", TASK_ID);
    let mut sub_read = signals::UART_READ.subscriber().unwrap();
    let pub_write = signals::UART_WRITE.publisher().unwrap();

    loop {
        let string = sub_read.next_message_pure().await;
        let mut result = false;
        //info!("{}: {}", TASK_ID, string.as_str());

        if string.starts_with("passthrough") || string.starts_with("1") {
            let mut throttle_settings = store::THROTTLE_SETTINGS.lock().await;
            if string.ends_with("1") {
                throttle_settings.passthrough = true;
            } else {
                throttle_settings.passthrough = false;
            }

            result = true;
        }

        if string.starts_with("increase_smooth_factor") || string.starts_with("2") {
            let mut throttle_settings = store::THROTTLE_SETTINGS.lock().await;
            let value = string.split_whitespace().next().unwrap();
            throttle_settings.increase_smooth_factor = value.parse::<i16>().unwrap();
            result = true;
        }

        if string.starts_with("decrease_smooth_factor") || string.starts_with("3") {
            let mut throttle_settings = store::THROTTLE_SETTINGS.lock().await;
            let value = string.split_whitespace().next().unwrap();
            throttle_settings.decrease_smooth_factor = value.parse::<i16>().unwrap();
            result = true;
        }

        if string.starts_with("no_throttle") || string.starts_with("4") {
            let mut throttle_settings = store::THROTTLE_SETTINGS.lock().await;
            let value = string.split_whitespace().next().unwrap();
            throttle_settings.no_throttle = value.parse::<i16>().unwrap();
            result = true;
        }

        if string.starts_with("full_throttle") || string.starts_with("5") {
            let mut throttle_settings = store::THROTTLE_SETTINGS.lock().await;
            let value = string.split_whitespace().next().unwrap();
            throttle_settings.full_throttle = value.parse::<i16>().unwrap();
            result = true;
        }

        if string.starts_with("deadband_min") || string.starts_with("6") {
            let mut throttle_settings = store::THROTTLE_SETTINGS.lock().await;
            let value = string.split_whitespace().next().unwrap();
            throttle_settings.deadband_min = value.parse::<i16>().unwrap();
            result = true;
        }

        if string.starts_with("deadband_max") || string.starts_with("7") {
            let mut throttle_settings = store::THROTTLE_SETTINGS.lock().await;
            let value = string.split_whitespace().next().unwrap();
            throttle_settings.deadband_max = value.parse::<i16>().unwrap();
            result = true;
        }

        if string.starts_with("speed_limit") || string.starts_with("8") {
            let mut throttle_settings = store::THROTTLE_SETTINGS.lock().await;
            let value = string.split_whitespace().next().unwrap();
            throttle_settings.speed_limit = value.parse::<i16>().unwrap();
            result = true;
        }


        // TODO
        if string.starts_with("settings") {
            if string.ends_with("write") {
               // signals::THROTTLE_SETTINGS_CHANGE.dyn_immediate_publisher().publish_immediate(true);
            }
            result = true;
        }

        if string.starts_with("power") {
            if string.ends_with("on") {
                signals::SWITCH_POWER.dyn_immediate_publisher().publish_immediate(true);
            } else {
                signals::SWITCH_POWER.dyn_immediate_publisher().publish_immediate(false);
            }

            result = true;
        }

        if string.starts_with("alarm") {
            if string.ends_with("on") {
                signals::ALARM_ENABLED.dyn_immediate_publisher().publish_immediate(true);
            } else if string.ends_with("play") {
                signals::ALARM_ALERT_ACTIVE.dyn_immediate_publisher().publish_immediate(true);
            } else {
                signals::ALARM_ALERT_ACTIVE.dyn_immediate_publisher().publish_immediate(false);
                signals::ALARM_ENABLED.dyn_immediate_publisher().publish_immediate(false);
            }

            result = true;
        }

        if string.starts_with("help") {
            // TODO: fix ble to be async? delay to avoid flooding
            Timer::after_millis(300).await; 

            let str: String<32> = String::try_from("1. passthrough 0/1").unwrap();
            pub_write.publish_immediate(str);
            
            Timer::after_millis(300).await; 

            let str: String<32> = String::try_from("2. increase_smooth_factor int").unwrap();
            pub_write.publish_immediate(str);

            Timer::after_millis(300).await; 

            let str: String<32> = String::try_from("3. decrease_smooth_factor int").unwrap();
            pub_write.publish_immediate(str);

            Timer::after_millis(300).await;

            let str: String<32> = String::try_from("4. no_throttle int - mv").unwrap();
            pub_write.publish_immediate(str);

            Timer::after_millis(300).await;

            let str: String<32> = String::try_from("5. full_throttle int - mv").unwrap();
            pub_write.publish_immediate(str);

            Timer::after_millis(300).await;

            let str: String<32> = String::try_from("6. deadband_min int - mv").unwrap();
            pub_write.publish_immediate(str);

            Timer::after_millis(300).await;

            let str: String<32> = String::try_from("7. deadband_max int - mv").unwrap();
            pub_write.publish_immediate(str);

            Timer::after_millis(300).await;

            let str: String<32> = String::try_from("8. speed_limit int - kmhr").unwrap();
            pub_write.publish_immediate(str);

            result = true;
        }
        
        // TODO: fix ble to be async? delay to avoid flooding
        Timer::after_millis(300).await; 

        // publish
        if result {
            let ok: String<32> = String::try_from("ok").unwrap();
            pub_write.publish_immediate(ok);
        } else {
            let error: String<32> = String::try_from("error").unwrap();
            pub_write.publish_immediate(error);
        }
    }
}