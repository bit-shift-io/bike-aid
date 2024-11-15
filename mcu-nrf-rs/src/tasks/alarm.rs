use core::future;
use crate::utils::{settings, signals};
use embassy_time::Timer;
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::mutex::Mutex;
use defmt::info;
use embassy_futures::{join::join, select::{select, Either}};

const TASK_ID: &str = "ALARM";
const WARN_INTERVAL: u64 = 10000; // 10 sec
static WARNING_COUNT: Mutex<ThreadModeRawMutex, u8> = Mutex::new(0);

#[embassy_executor::task]
pub async fn task() {
    info!("{}", TASK_ID);

    // alarm mode
    let mut rec_alarm_mode = signals::ALARM_MODE.receiver().unwrap();
    let mut state_alarm_mode = AlarmMode::Off;

    loop { 
        match select(rec_alarm_mode.changed(), run(state_alarm_mode)).await {
            Either::First(b) => { state_alarm_mode = b },
            Either::Second(_) => {}
        }
    }
}


async fn run(alarm_mode: AlarmMode) {
    if alarm_mode == AlarmMode::On && alarm_mode != AlarmMode::Siren {
        info!("start alarm detection");
        signals::send_ble(signals::BleHandles::AlarmOn, &[true as u8]);
        join(alarm(), warning_cooldown()).await;
    }
    else if alarm_mode == AlarmMode::Off { stop().await };

    future::pending().await // wait/yield forever doing nothing
}


async fn alarm() {
    let send_alarm_mode = signals::ALARM_MODE.sender();
    let mut rec_motion_detected = signals::MOTION_DETECTED.receiver().unwrap();
    let mut rec_alarm_settings = settings::ALARM_SETTINGS.receiver().unwrap();
    let settings = rec_alarm_settings.try_get().unwrap();

    //let send_motion_detected = signals::MOTION_DETECTED.sender();
    let send_piezo = signals::PIEZO_MODE.sender();
    
    
    // TODO: want to time limit the warnings to every xx seconds
    loop {
        // motion detected
        if rec_motion_detected.changed().await {
            
            let warn_count = {
                let c = *WARNING_COUNT.lock().await;
                *WARNING_COUNT.lock().await = c + 1;
                c
            };

            if warn_count > settings.warning_count {
                // alarm
                info!("ALARM!");
                send_alarm_mode.send(AlarmMode::Siren);
                send_piezo.send(signals::PiezoModeType::Alarm);
            } else {
                // warning
                send_piezo.send(signals::PiezoModeType::Warning);
                send_alarm_mode.send(AlarmMode::Warning(0));
                info!("{}: alarm warning", TASK_ID);
            }

            // reset motion detected, dont think this is needed with watch
            //send_motion_detected.clear();
        };
    }
}


async fn stop() {
    // only send if changed
    signals::PIEZO_MODE.sender().send(signals::PiezoModeType::None);
    if signals::MOTION_DETECTED.try_get().unwrap() {
        signals::MOTION_DETECTED.sender().send(false);
    }
    if signals::ALARM_MODE.try_get().unwrap() != AlarmMode::Off {
        signals::send_ble(signals::BleHandles::AlarmOn, &[false as u8]);
    }
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


#[derive(Clone, Copy, PartialEq)]
pub enum AlarmMode {
    Off,
    On,
    Warning(u8),
    Siren,
}