use embassy_executor::Spawner;
use embassy_nrf::gpio::AnyPin;
use embassy_nrf::gpio::{Input, Pull};
use embassy_time::Timer;
use embassy_sync::channel::Channel;
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::mutex::Mutex;
use defmt::info;

const TASK_ID: &str = "ALARM";
const WARN_INTERVAL: u64 = 10000; // 10 sec
const WARNINGS: u8 = 3;
const MOTION_SENSITIVITY: u16 = 40; // to account for sensor vibration
const MOTION_TIMEOUT: u64 = 2000; // ms to reset motion sensitivity

static MOTION_DETECTED: Channel<ThreadModeRawMutex, bool, 1> = Channel::new();
static WARNING_COUNT: Mutex<ThreadModeRawMutex, u8> = Mutex::new(0);

#[embassy_executor::task]
pub async fn alarm (
    spawner: Spawner,
    pin: AnyPin
) {
    info!("{}", TASK_ID);
    //let send_alarm = signals::ALARM.sender();
    
    // spawn sub tasks
    spawner.must_spawn(warning_cooldown());
    spawner.must_spawn(motion_detection(pin));
    
    loop {
        // motion detected
        if MOTION_DETECTED.receive().await {
            let warn_count = WARNING_COUNT.lock().await;

            if *warn_count > WARNINGS {
                info!("ALARM!");
                //send_alarm.send(true);
            } 

            // reset motion detected
            MOTION_DETECTED.send(false).await;
        };
    }
}


#[embassy_executor::task]
async fn motion_detection (
    pin: AnyPin
) {
    // all this task does it detect motion
    let mut trigger_count = 0;
    let mut last_time = 0;
    let mut pin_state = Input::new(pin, Pull::Down); // low

    loop {
        pin_state.wait_for_high().await;
        trigger_count += 1;
        let time = embassy_time::Instant::now().as_millis();

          // every interval check
        if time - last_time > MOTION_TIMEOUT {
            last_time = time;

            // sensitivity here
            if trigger_count > MOTION_SENSITIVITY {
                // increment mutex
                let mut warn_count = WARNING_COUNT.lock().await;
                *warn_count += 1;
                // send signal / notify
                MOTION_DETECTED.send(true).await;
            }

            trigger_count = 0;
        }
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