use crate::utils::{i2c, settings, signals};
use embassy_embedded_hal::shared_bus::asynch::i2c::I2cDevice;
use embassy_nrf::{peripherals::TWISPI0, twim::Twim};
use defmt::info;
use mpu6050_async::Mpu6050;
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::mutex;
use embassy_time::{Delay, Timer};
use embassy_futures::select::select;
use num_traits::Float;

const TASK_ID : &str = "GYROSCOPE";
const INTERVAL: u64 = 1000; // ms
const ADDRESS: u8 = 0x68;

#[embassy_executor::task]
pub async fn task(
    i2c_bus: &'static mutex::Mutex<ThreadModeRawMutex, Twim<'static, TWISPI0>>
) {
    info!("{}", TASK_ID);

    // alarm on/off
    let mut rec_alarm_mode = signals::ALARM_MODE.receiver().unwrap();

    loop { 
        let alarm_mode = rec_alarm_mode.changed().await;
        if alarm_mode != signals::AlarmModeType::Off && alarm_mode != signals::AlarmModeType::Siren {
            select(rec_alarm_mode.changed(), motion_detection(i2c_bus)).await;
        }
    }
}


async fn motion_detection(i2c_bus: &'static mutex::Mutex<ThreadModeRawMutex, Twim<'static, TWISPI0>>) {
    let i2c_bus_temp = i2c::get_i2c_bus();
    // check if device available
    if !i2c::device_available(i2c_bus_temp, ADDRESS).await {
        info!("{}: end", TASK_ID);
        return;
    }

    // init device
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
    
    let mut rec_alarm_settings = settings::ALARM_SETTINGS.receiver().unwrap();
    let send_motion = signals::MOTION_DETECTED.sender();
    let mut last_gyro = mpu.get_gyro().await.unwrap();
    let mut last_acc_angles = mpu.get_acc_angles().await.unwrap();

    loop {
        Timer::after_millis(INTERVAL).await;

        let settings = rec_alarm_settings.try_get().unwrap();
        let mut motion_detected = false;

        // get roll and pitch estimate
        let acc_angles = mpu.get_acc_angles().await.unwrap();
        let x_acc_delta = (acc_angles.0 - last_acc_angles.0).abs();
        let y_acc_delta = (acc_angles.1 - last_acc_angles.1).abs();
        if x_acc_delta > settings.angle_sensitivity || y_acc_delta > settings.angle_sensitivity {
            motion_detected = true;
            info!("{}: angles detected", TASK_ID);
        }

        // get gyro data, scaled with sensitivity
        let gyro = mpu.get_gyro().await.unwrap();
        let x_gyro_delta = (gyro.0 - last_gyro.0).abs();
        let y_gyro_delta = (gyro.1 - last_gyro.1).abs();
        let z_gyro_delta = (gyro.2 - last_gyro.2).abs();
        if x_gyro_delta > settings.gyro_sensitivity || y_gyro_delta > settings.gyro_sensitivity || z_gyro_delta > settings.gyro_sensitivity {
            motion_detected = true;
            info!("{}: gyro detected", TASK_ID);
        }
        
        // get accelerometer data, scaled with sensitivity
        let acc = mpu.get_acc().await.unwrap(); // in G's
        // tuple -> array
        // map abs on each element
        // find the max value with parital compare
        let acc_abs = [acc.0, acc.1, acc.2].iter().map(|x| x.abs()).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap(); 
        if acc_abs > settings.acc_sensitivity {
            motion_detected = true;
            info!("{}: acc detected", TASK_ID);
        }

        // for debug
        //info!(" acc: {} | r/p: {} {} | gyro: {} {} {}", acc_abs, x_acc_delta, y_acc_delta, x_gyro_delta, y_gyro_delta, z_gyro_delta);
        //info!("acc: {} | gyro: {} | r/p: {}", Debug2Format(&acc), Debug2Format(&gyro), Debug2Format(&acc_angles));

        if motion_detected {
            info!(" acc: {} | r/p: {} {} | gyro: {} {} {}", acc_abs, x_acc_delta, y_acc_delta, x_gyro_delta, y_gyro_delta, z_gyro_delta);
            send_motion.send(true);
        }

        last_gyro = gyro;
        last_acc_angles = acc_angles;
    }
}