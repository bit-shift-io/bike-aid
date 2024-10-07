use crate::utils::signals;
use embassy_embedded_hal::shared_bus::blocking::i2c::I2cDevice;
use embassy_nrf::{peripherals::TWISPI0, twim::Twim};
use defmt::*;
use embassy_futures::select::{select, Either};
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::blocking_mutex::Mutex;
use core::cell::RefCell;
use embassy_time::{Delay, Timer};
use mpu6050::*;

const TASK_ID : &str = "GYROSCOPE";
// TODO: move these to settings?
const ACC_SENSITIVITY: f32 = 0.9;
const GYRO_SENSITIVITY: f32 = 0.8;
const ANGLE_SENSITIVITY: f32 = 0.1;
const INTERVAL: u64 = 500;

#[embassy_executor::task]
pub async fn task(
    i2c_bus: &'static Mutex<NoopRawMutex, RefCell<Twim<'static, TWISPI0>>>
) {
    info!("{}: start", TASK_ID);

    let mut sub = signals::ALARM_ENABLED.subscriber().unwrap();
    let mut state = false;

    loop { 
        if let Some(b) = sub.try_next_message_pure() {state = b}
        match state {
            true => {
                let sub_future = sub.next_message_pure();
                let task_future = run(i2c_bus);
                match select(sub_future, task_future).await {
                    Either::First(val) => { state = val; }
                    Either::Second(_) => { Timer::after_secs(60).await; } // retry
                }
            },
            false => { state = sub.next_message_pure().await; }
        }
    }
}


async fn run(i2c_bus: &'static Mutex<NoopRawMutex, RefCell<Twim<'static, TWISPI0>>>) {
    let i2c = I2cDevice::new(i2c_bus);
    let mut mpu = Mpu6050::new(i2c);
    let mut delay = Delay;
    let result = mpu.init(&mut delay);
    match result {
        Ok(()) => {},
        Err(_e) => {
            info!("{}: device error", TASK_ID);
            return
        }, // unable to communicate with device
    }

    // sensitivity / range
    //let _ = mpu.set_gyro_range(GyroRange::D250); // default GyroRange::D250
    //let _ = mpu.set_accel_range(AccelRange::G2); // default AccelRange::G2
    //let _ = mpu.set_accel_hpf(ACCEL_HPF::_RESET); // default ACCEL_HPF::_RESET
    //mpu.setup_motion_detection().unwrap();

    let pub_motion = signals::ALARM_MOTION_DETECTED.publisher().unwrap();
    let mut last_gyro = mpu.get_gyro().unwrap();
    let mut last_acc_angles = mpu.get_acc_angles().unwrap();

    loop {
        Timer::after_millis(INTERVAL).await;
        let mut motion_detected = false;

        // get roll and pitch estimate
        let acc_angles = mpu.get_acc_angles().unwrap();
        let x_acc_delta = acc_angles.x - last_acc_angles.x;
        let y_acc_delta = acc_angles.y - last_acc_angles.y;
        if x_acc_delta > ANGLE_SENSITIVITY || y_acc_delta > ANGLE_SENSITIVITY {
            motion_detected = true;
            //info!("{}: angles detected", TASK_ID);
        }

        // get gyro data, scaled with sensitivity
        let gyro = mpu.get_gyro().unwrap();
        let x_gyro_delta = gyro.x - last_gyro.x;
        let y_gyro_delta = gyro.y - last_gyro.y;
        let z_gyro_delta = gyro.z - last_gyro.z;
        if x_gyro_delta > GYRO_SENSITIVITY || y_gyro_delta > GYRO_SENSITIVITY || z_gyro_delta > GYRO_SENSITIVITY {
            motion_detected = true;
            //info!("{}: gyro detected", TASK_ID);
        }
        
        // get accelerometer data, scaled with sensitivity
        let acc = mpu.get_acc().unwrap(); // in G's
        if acc.abs().amax() > ACC_SENSITIVITY {
            motion_detected = true;
            //info!("{}: acc detected", TASK_ID);
        }

        // for debug
        //info!("{} | {}", x_acc_delta, y_acc_delta);
        //info!("{} | {} | {}", x_gyro_delta, y_gyro_delta, z_gyro_delta);
        //info!("{} | {}", acc.abs().amin(), acc.abs().amax());
        //info!("acc: {} | gyro: {} | r/p: {}", Debug2Format(&acc), Debug2Format(&gyro), Debug2Format(&acc_angles));

        if motion_detected {
            pub_motion.publish_immediate(true);
        }

        last_gyro = gyro;
        last_acc_angles = acc_angles;
    }
}