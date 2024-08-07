use crate::utils::signals;
use defmt::*;
use heapless::String;

const TASK_ID: &str = "CLI";

#[embassy_executor::task]
pub async fn cli () {
    info!("{}: start", TASK_ID);
    let mut sub_read = signals::UART_READ.subscriber().unwrap();
    let pub_write = signals::UART_WRITE.publisher().unwrap();

    loop {
        let string = sub_read.next_message_pure().await;

        /*
        // debug new line endings and stuff with the strings
        info!("{}: {}", TASK_ID, string);
        info!("{}: length: {}", TASK_ID, string.len());
        info!("{}: ends with on: {}", TASK_ID, string.ends_with("on"));
        info!("{}: str > array: {}", TASK_ID, str_to_array(string));
        */

        if string.starts_with("power") {
            if string.ends_with("on") {
                signals::SWITCH_POWER.dyn_immediate_publisher().publish_immediate(true);
            } else {
                signals::SWITCH_POWER.dyn_immediate_publisher().publish_immediate(false);
            }
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
        }
        
        // publish
        let ok: String<32> = String::try_from("ok").unwrap();
        pub_write.publish_immediate(ok);
    }
}