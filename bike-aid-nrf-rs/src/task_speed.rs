use crate::signals;
use embassy_nrf::gpio::{AnyPin, Level};
use embassy_time::{Duration, Timer};
use embassy_nrf::gpio::{Input, Pull};
use defmt::*;

static TASK_ID : &str = "SPEED";
static WHEEL_CIRCUMFERENCE : f32 = 1105.0; // 12.5inch diameter -> 317.5mm diameter -> 997.46mm circumference
static SPEED_SMOOTH_FACTOR : f32 = 0.3;
static TICKS_PER_ROTATION : u32 = 12; // TODO:how many times the sensor will tick per rotation

#[embassy_executor::task]
pub async fn speed (
    pin : AnyPin
) {
    let pub_instant_speed = signals::INSTANT_SPEED.publisher().unwrap();
    let pub_smooth_speed = signals::SMOOTH_SPEED.publisher().unwrap();
    let pub_wheel_rotations = signals::WHEEL_ROTATIONS.publisher().unwrap();
    let pub_odometer = signals::ODOMETER.publisher().unwrap();


    let mut rotations = 0;
    let mut last_rotation_time = 0;
    let mut rotation_time = 0;
    let mut last_level  = Level::Low; // default low
    let mut smooth_speed = 0.0;
    let mut pin_state = Input::new(pin, Pull::Down); // low

    info!("{} : Entering main loop",TASK_ID);
    loop {
        pin_state.wait_for_high().await;
        info!("Button pressed!");

        pin_state.wait_for_low().await;
        info!("Button released!");

        let mut pin_level = pin_state.get_level();

        if (pin_level == last_level || pin_level == Level::Low) {
            continue;
        }

        // pin high, another rotation completed
        if (pin_level == Level::High) {
            let time = embassy_time::Instant::now().as_micros();

            // rotations
            rotations += 1;
            pub_wheel_rotations.publish_immediate(rotations);

            // odometer
            let odometer = (WHEEL_CIRCUMFERENCE * rotations as f32 * 0.0036) as u8;  // round
            pub_odometer.publish_immediate(odometer);

            // speed
            last_rotation_time = rotation_time;
            rotation_time = time;

            // calc instant speed
            let delta_time: f32 = (rotation_time - last_rotation_time) as f32;
            if (delta_time > 20.0 && delta_time < 5000.0) {
                // mm per second -> kms (1mm/s = 0.0036km/s)
                let instant_speed: f32 = (1000.0 / delta_time) * WHEEL_CIRCUMFERENCE * 0.0036;
                pub_instant_speed.publish_immediate(instant_speed as u32); // round

                // calculate smooth speed
                let delta_speed : f32 = instant_speed - smooth_speed; // calc difference btween speeds
                let speed_adjust = delta_speed * SPEED_SMOOTH_FACTOR; // todo: multiply by delta time, so faster speeds are adjusted faster?
                smooth_speed += speed_adjust;
                pub_smooth_speed.publish_immediate(smooth_speed as u32); // round
            }
            
        }
        last_level = pin_level;
    }
}