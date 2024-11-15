use core::future;
use crate::utils::signals;
use embassy_time::{Instant, Timer};
use defmt::info;
use embassy_futures::{join::join, select::{select, Either}};

const TASK_ID: &str = "ALARM";
const WARN_INTERVAL: u64 = 10000; // 10 sec
const MOTION_INTERVAL: u64 = 1000; // 1 sec
const WARN_COUNT: u8 = 2; // 3 warnings

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
    let send_piezo = signals::PIEZO_MODE.sender();
    let send_alarm_mode = signals::ALARM_MODE.sender();
    let future_alarm_tasks = join(motion_detection(), warning_cooldown());

    match alarm_mode {
        AlarmMode::Off => {
            send_piezo.send(signals::PiezoModeType::None);
            signals::send_ble(signals::BleHandles::AlarmOn, &[false as u8]);
        },
        AlarmMode::On => {
            info!("start alarm detection");
            signals::send_ble(signals::BleHandles::AlarmOn, &[true as u8]);
            future_alarm_tasks.await;
        }
        AlarmMode::Warning(_) => {
            send_piezo.send(signals::PiezoModeType::Warning);
            send_alarm_mode.send(AlarmMode::Warning(0));
            info!("{}: alarm warning", TASK_ID);
            future_alarm_tasks.await;
        },
        AlarmMode::Siren => {
            info!("ALARM!");
            send_alarm_mode.send(AlarmMode::Siren);
            send_piezo.send(signals::PiezoModeType::Alarm);
        }
    }

    future::pending().await // wait/yield forever doing nothing
}


async fn motion_detection() {
    let mut rec_motion_detected = signals::MOTION_DETECTED.receiver().unwrap();
    let last_time = Instant::now();

    loop {
        if rec_motion_detected.changed().await {
            let time = Instant::now();
            if time.duration_since(last_time).as_millis() > MOTION_INTERVAL {
                increment_alarm().await;
                // dont need to keep track of last_time as increment will restart this function
            }
        };
    }
}


async fn increment_alarm() {
    let rec_alarm_mode = signals::ALARM_MODE.try_get().unwrap();
    let new_alarm_mode = match rec_alarm_mode {
        AlarmMode::Off => AlarmMode::On,
        AlarmMode::On => AlarmMode::Warning(0),
        AlarmMode::Warning(level) if level < WARN_COUNT => AlarmMode::Warning(level + 1),
        _ => AlarmMode::Siren,
    };

    signals::ALARM_MODE.sender().send(new_alarm_mode);
}


async fn warning_cooldown() {
    loop {
        Timer::after_millis(WARN_INTERVAL).await;
        let rec_alarm_mode = signals::ALARM_MODE.try_get().unwrap();
        if let AlarmMode::Warning(level) = rec_alarm_mode {
            if level > 0 {
                signals::ALARM_MODE.sender().send(AlarmMode::Warning(level - 1));
            }
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