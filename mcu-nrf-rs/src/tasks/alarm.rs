use crate::utils::{settings, signals};
use embassy_time::Timer;
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::mutex::Mutex;
use defmt::info;
use embassy_futures::{join::join, select::select};

const TASK_ID: &str = "ALARM";
const WARN_INTERVAL: u64 = 10000; // 10 sec
static WARNING_COUNT: Mutex<ThreadModeRawMutex, u8> = Mutex::new(0);

#[embassy_executor::task]
pub async fn task() {
    info!("{}", TASK_ID);

    // alarm on/off
    let mut rec = signals::ALARM_ENABLED.receiver().unwrap();

    loop { 
        if rec.changed().await {
            signals::send_ble(signals::BleHandles::AlarmOn, &[true as u8]);
            let watch_future = rec.changed();
            let task1_future = alarm();
            let task2_future = warning_cooldown();
            let task_future = join(task1_future, task2_future);
            select(watch_future, task_future).await;
            stop().await;
        }
    }
}

async fn alarm() {
    let send_alarm = signals::ALARM_ALERT_ACTIVE.sender();
    let mut rec_motion = signals::ALARM_MOTION_DETECTED.receiver().unwrap();
    let send_motion = signals::ALARM_MOTION_DETECTED.sender();
    let send_piezo = signals::PIEZO_MODE.sender();

    // TODO: want to time limit the warnings to every xx seconds
    loop {
        // motion detected
        if rec_motion.changed().await {
            info!("{}: motion detected", TASK_ID);
            let settings = { *settings::ALARM_SETTINGS.lock().await };
            let mut warn_count = WARNING_COUNT.lock().await;
            *warn_count += 1;

            if *warn_count > settings.warning_count {
                // alarm
                info!("ALARM!");
                send_alarm.send(true);
                send_piezo.send(signals::PiezoModeType::Alarm);
            } else {
                // warning
                send_piezo.send(signals::PiezoModeType::Warning);
            }

            // reset motion detected
            send_motion.clear();
        };
    }
}


async fn stop() {
    let send_alarm = signals::ALARM_ALERT_ACTIVE.sender();
    let send_motion = signals::ALARM_MOTION_DETECTED.sender();
    let send_piezo = signals::PIEZO_MODE.sender();
    send_alarm.send(false);
    send_piezo.send(signals::PiezoModeType::None);
    send_motion.send(false);
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