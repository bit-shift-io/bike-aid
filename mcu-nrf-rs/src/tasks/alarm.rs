use crate::utils::signals;
use embassy_executor::Spawner;
use embassy_time::Timer;
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::mutex::Mutex;
use defmt::*;

const TASK_ID: &str = "ALARM";
const WARN_INTERVAL: u64 = 10000; // 10 sec
const WARNINGS: u8 = 3;
static WARNING_COUNT: Mutex<ThreadModeRawMutex, u8> = Mutex::new(0);

#[embassy_executor::task]
pub async fn task(
    spawner: Spawner,
) {
    info!("{}: start", TASK_ID);
    let pub_alarm = signals::ALARM_ALERT_ACTIVE.publisher().unwrap();
    let mut sub_motion = signals::ALARM_MOTION_DETECTED.subscriber().unwrap();
    let pub_motion = signals::ALARM_MOTION_DETECTED.publisher().unwrap();

    // spawn sub tasks
    spawner.must_spawn(warning_cooldown());

    // TODO: want to time limit the warnings to every xx seconds

    loop {
        // motion detected
        if sub_motion.next_message_pure().await {
            info!("{}: motion detected", TASK_ID);
            let mut warn_count = WARNING_COUNT.lock().await;
            *warn_count += 1;

            if *warn_count > WARNINGS {
                info!("ALARM!");
                pub_alarm.publish_immediate(true);
            } 

            // reset motion detected
            pub_motion.clear();
        };
    }
}


#[embassy_executor::task]
async fn warning_cooldown() {
    loop {
        Timer::after_millis(WARN_INTERVAL).await;
        let mut warn_count = WARNING_COUNT.lock().await;
        if *warn_count > 0 {
            *warn_count -= 1;
        }
    }
}