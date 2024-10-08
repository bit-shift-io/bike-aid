use crate::utils::signals;
use embassy_time::Timer;
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::mutex::Mutex;
use defmt::*;
use embassy_futures::{join::join, select::{select, Either}};

const TASK_ID: &str = "ALARM";
const WARN_INTERVAL: u64 = 10000; // 10 sec
const WARNINGS: u8 = 3;
static WARNING_COUNT: Mutex<ThreadModeRawMutex, u8> = Mutex::new(0);

#[embassy_executor::task]
pub async fn task() {
    info!("{}: start", TASK_ID);

    let mut sub = signals::ALARM_ENABLED.subscriber().unwrap();
    let mut state = false;

    loop { 
        if let Some(b) = sub.try_next_message_pure() {state = b}
        match state {
            true => {
                let sub_future = sub.next_message_pure();
                let task1_future = run();
                let task2_future = warning_cooldown();
                let task_future = join(task1_future, task2_future);
                match select(sub_future, task_future).await {
                    Either::First(val) => { state = val; }
                    Either::Second(_) => { Timer::after_secs(60).await; } // retry
                }
            },
            false => {
                stop().await; // user turned alarm off
                state = sub.next_message_pure().await; 
            }
        }
    }
}

async fn run() {
    let pub_alarm = signals::ALARM_ALERT_ACTIVE.publisher().unwrap();
    let mut sub_motion = signals::ALARM_MOTION_DETECTED.subscriber().unwrap();
    let pub_motion = signals::ALARM_MOTION_DETECTED.publisher().unwrap();
    let pub_piezo = signals::PIEZO_MODE.publisher().unwrap();

    // TODO: want to time limit the warnings to every xx seconds

    loop {
        // motion detected
        if sub_motion.next_message_pure().await {
            info!("{}: motion detected", TASK_ID);
            let mut warn_count = WARNING_COUNT.lock().await;
            *warn_count += 1;

            if *warn_count > WARNINGS {
                // alarm
                info!("ALARM!");
                pub_alarm.publish_immediate(true);
                pub_piezo.publish_immediate(signals::PiezoModeType::Alarm);
            } else {
                // warning
                pub_piezo.publish_immediate(signals::PiezoModeType::Warning);
            }

            // reset motion detected
            pub_motion.clear();
        };
    }
}



async fn stop() {
    let pub_alarm = signals::ALARM_ALERT_ACTIVE.publisher().unwrap();
    let pub_motion = signals::ALARM_MOTION_DETECTED.publisher().unwrap();
    let pub_piezo = signals::PIEZO_MODE.publisher().unwrap();
    pub_alarm.publish_immediate(false);
    pub_piezo.publish_immediate(signals::PiezoModeType::None);
    pub_motion.publish_immediate(false);
}


async fn warning_cooldown() {
    loop {
        Timer::after_millis(WARN_INTERVAL).await;
        let mut warn_count = WARNING_COUNT.lock().await;
        if *warn_count > 0 {
            *warn_count -= 1;
        }
    }
}