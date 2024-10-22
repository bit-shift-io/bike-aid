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
    info!("{}", TASK_ID);

    let mut rec = signals::ALARM_ENABLED_WATCH.receiver().unwrap();
    let mut state = false;
    let mut init = false;

    loop { 
        if let Some(b) = rec.try_changed() {state = b}
        match state {
            true => {
                signals::send_ble(signals::BleHandles::AlarmOn, &[true as u8]).await;
                let watch_future = rec.changed();
                let task1_future = run();
                let task2_future = warning_cooldown();
                let task_future = join(task1_future, task2_future);
                match select(watch_future, task_future).await {
                    Either::First(val) => { state = val; }
                    Either::Second(_) => { Timer::after_secs(60).await; } // retry
                }
            },
            false => {
                if init {
                    signals::send_ble(signals::BleHandles::AlarmOn, &[false as u8]).await;
                    stop().await; // user turned alarm off
                } else { init = true; }
                state = rec.changed().await; 
            }
        }
    }
}

async fn run() {
    let send_alarm = signals::ALARM_ALERT_ACTIVE_WATCH.sender();
    let mut rec_motion = signals::ALARM_MOTION_DETECTED_WATCH.receiver().unwrap();
    let send_motion = signals::ALARM_MOTION_DETECTED_WATCH.sender();
    let send_piezo = signals::PIEZO_MODE_WATCH.sender();

    // TODO: want to time limit the warnings to every xx seconds
    loop {
        // motion detected
        if rec_motion.changed().await {
            info!("{}: motion detected", TASK_ID);
            let mut warn_count = WARNING_COUNT.lock().await;
            *warn_count += 1;

            if *warn_count > WARNINGS {
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
    let send_alarm = signals::ALARM_ALERT_ACTIVE_WATCH.sender();
    let send_motion = signals::ALARM_MOTION_DETECTED_WATCH.sender();
    let send_piezo = signals::PIEZO_MODE_WATCH.sender();
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