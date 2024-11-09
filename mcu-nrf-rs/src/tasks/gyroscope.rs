use crate::utils::{i2c, signals};
use embassy_embedded_hal::shared_bus::asynch::i2c::I2cDevice;
use embassy_nrf::{peripherals::TWISPI0, twim::Twim};
use defmt::info;
use mpu6050_async::Mpu6050;
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::mutex;
use embassy_time::{Delay, Timer};
use embassy_futures::select::{select, Either};
use num_traits::Float;

const TASK_ID : &str = "GYROSCOPE";
// TODO: move these to settings?
const ACC_SENSITIVITY: f32 = 0.9;
const GYRO_SENSITIVITY: f32 = 0.8;
const ANGLE_SENSITIVITY: f32 = 0.1;
const INTERVAL: u64 = 500;
const ADDRESS: u8 = 0x68;

#[embassy_executor::task]
pub async fn task(
    i2c_bus: &'static mutex::Mutex<ThreadModeRawMutex, Twim<'static, TWISPI0>>
) {
    info!("{}", TASK_ID);

    if !i2c::device_available(i2c_bus, ADDRESS).await {
        info!("{}: end", TASK_ID);
        return;
    }

    let mut sub = signals::ALARM_ENABLED.receiver().unwrap();
    let mut state = false;

    loop { 
        if let Some(b) = sub.try_changed() {state = b}
        match state {
            true => {
                let rec_future = sub.changed();
                let task_future = run(i2c_bus);
                match select(rec_future, task_future).await {
                    Either::First(val) => { state = val; }
                    Either::Second(_) => { Timer::after_secs(60).await; } // retry
                }
            },
            false => { state = sub.changed().await; }
        }
    }
}


async fn run(i2c_bus: &'static mutex::Mutex<ThreadModeRawMutex, Twim<'static, TWISPI0>>) {
    let i2c = I2cDevice::new(i2c_bus);
    let mut mpu = Mpu6050::new(i2c);
    match mpu.init(&mut Delay).await {
        Ok(()) => {},
        Err(_e) => {
            info!("{}: device error", TASK_ID);
            return;
        }, // unable to communicate with device
    }

    // sensitivity / range
    //let _ = mpu.set_gyro_range(GyroRange::D250); // default GyroRange::D250
    //let _ = mpu.set_accel_range(AccelRange::G2); // default AccelRange::G2
    //let _ = mpu.set_accel_hpf(ACCEL_HPF::_RESET); // default ACCEL_HPF::_RESET
    //mpu.setup_motion_detection().unwrap();

    let send_motion = signals::ALARM_MOTION_DETECTED.sender();
    let mut last_gyro = mpu.get_gyro().await.unwrap();
    let mut last_acc_angles = mpu.get_acc_angles().await.unwrap();

    loop {
        Timer::after_millis(INTERVAL).await;
        let mut motion_detected = false;

        // get roll and pitch estimate
        let acc_angles = mpu.get_acc_angles().await.unwrap();
        let x_acc_delta = acc_angles.0 - last_acc_angles.0;
        let y_acc_delta = acc_angles.1 - last_acc_angles.1;
        if x_acc_delta > ANGLE_SENSITIVITY || y_acc_delta > ANGLE_SENSITIVITY {
            motion_detected = true;
            //info!("{}: angles detected", TASK_ID);
        }

        // get gyro data, scaled with sensitivity
        let gyro = mpu.get_gyro().await.unwrap();
        let x_gyro_delta = gyro.0 - last_gyro.0;
        let y_gyro_delta = gyro.1 - last_gyro.1;
        let z_gyro_delta = gyro.2 - last_gyro.2;
        if x_gyro_delta > GYRO_SENSITIVITY || y_gyro_delta > GYRO_SENSITIVITY || z_gyro_delta > GYRO_SENSITIVITY {
            motion_detected = true;
            //info!("{}: gyro detected", TASK_ID);
        }
        
        // get accelerometer data, scaled with sensitivity
        let acc = mpu.get_acc().await.unwrap(); // in G's
        // tuple -> array
        // map abs on each element
        // find the max value with parital compare
        let acc_abs = [acc.0, acc.1, acc.2].iter().map(|x| x.abs()).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap(); 
        if acc_abs > ACC_SENSITIVITY {
            motion_detected = true;
            //info!("{}: acc detected", TASK_ID);
        }

        // for debug
        //info!("{} | {}", x_acc_delta, y_acc_delta);
        //info!("{} | {} | {}", x_gyro_delta, y_gyro_delta, z_gyro_delta);
        //info!("{} | {}", acc.abs().amin(), acc.abs().amax());
        //info!("acc: {} | gyro: {} | r/p: {}", Debug2Format(&acc), Debug2Format(&gyro), Debug2Format(&acc_angles));

        if motion_detected {
            send_motion.send(true);
        }

        last_gyro = gyro;
        last_acc_angles = acc_angles;
    }
}