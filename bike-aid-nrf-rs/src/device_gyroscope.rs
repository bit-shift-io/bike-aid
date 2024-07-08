use crate::signals;
use device::{AccelRange, GyroRange, ACCEL_HPF, ACCEL_SENS};
use embassy_embedded_hal::shared_bus::blocking::i2c::I2cDevice;
use embassy_nrf::{peripherals::TWISPI0, twim::Twim};
use defmt::*;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_time::{Delay, Timer};
use mpu6050::{*, device::MOT_DETECT_STATUS};

static DEVICE_ID : &str = "GYROSCOPE";

#[embassy_executor::task]
pub async fn gyroscope (
    i2c: I2cDevice<'static,NoopRawMutex, Twim<'static,TWISPI0>>
) {
    let pub_throttle = signals::THROTTLE_IN.publisher().unwrap();
    let mut mpu = Mpu6050::new(i2c);
    let mut delay = Delay;
    let result = mpu.init(&mut delay);
    match result {
        Ok(()) => {},
        Err(e) => {
            info!("{} : device error", DEVICE_ID);
            return
        }, // unable to communicate with device
    }

    // sensitivity / range
    //let _ = mpu.set_gyro_range(GyroRange::D250); // default GyroRange::D250
    //let _ = mpu.set_accel_range(AccelRange::G2); // default AccelRange::G2
    //let _ = mpu.set_accel_hpf(ACCEL_HPF::_RESET); // default ACCEL_HPF::_RESET


    mpu.setup_motion_detection().unwrap();

    let INTERVAL = 1000; // 1 sec
    let WARN_INTERVAL = 10; // 10 x 1 sec = 10 sec
    let SENSITIVITY = 2;
    let WARNINGS = 3;

    let mut trigger_count: u8 = 0;
    let mut warn_count = 0;
    let mut warn_interval_count = 0;

    info!("{} : Entering main loop", DEVICE_ID);
    loop {
        Timer::after_millis(10).await;

        // check for motion
        if mpu.get_motion_detected().unwrap() {
            info!("Motion by axes: {:b}", mpu.read_byte(MOT_DETECT_STATUS::ADDR).unwrap());
            trigger_count += 1;
        }
        /*
        // sensitivity here, or use the gyro settings
        if trigger_count > SENSITIVITY {
            warn_count += 1;
            info!("warn");
        }

        // check the warn count
        if warn_count > WARNINGS {
            //todo: set active alarm, play till user resets
            info!("ALARM!");
        }

        if warn_count == 0 {
            continue;
        }

        // increment warn interval count
        warn_interval_count += 1;

        // reduce warn count if within time
        warn_interval_count += 1;
        if warn_interval_count >= WARN_INTERVAL {
            warn_count -= 1;
            warn_interval_count = 0;
        }

         */

        /*
        // not sure what this is for?
        if count > 5 {
            mpu.reset_device(&mut delay).unwrap();
            break;
        }
         */
    }

    /*
    
    // get roll and pitch estimate
    let acc = mpu.get_acc_angles().unwrap();
    defmt::info!("r/p: {}", defmt::Debug2Format(&acc));

    // get temp
    let temp = mpu.get_temp().unwrap();
    defmt::info!("temp: {}°C", defmt::Debug2Format(&temp));

    // get gyro data, scaled with sensitivity
    let gyro = mpu.get_gyro().unwrap();
    defmt::info!("gyro: {}", defmt::Debug2Format(&gyro));

    // get accelerometer data, scaled with sensitivity
    let acc = mpu.get_acc().unwrap();
    defmt::info!("acc: {}", defmt::Debug2Format(&acc));
     */
}