use crate::utils::{functions::{bytes_to_string, str_to_array}, signals};
use defmt::*;

const TASK_ID: &str = "CLI";

#[embassy_executor::task]
pub async fn cli () {
    info!("{}: start", TASK_ID);
    let mut sub_read = signals::UART_READ.subscriber().unwrap();
    let pub_write = signals::UART_WRITE.publisher().unwrap();

    loop {
        let input = sub_read.next_message_pure().await;

        let string = bytes_to_string(&input);
        info!("{}: {}", TASK_ID, string);
        if string.starts_with("power") {
            if string.ends_with("on") {
                signals::SWITCH_POWER.dyn_immediate_publisher().publish_immediate(true);
            } else {
                signals::SWITCH_POWER.dyn_immediate_publisher().publish_immediate(false);
            }
        }
        
        // publish
        pub_write.publish_immediate(str_to_array("ok\n"));
    }
}