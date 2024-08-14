use crate::utils::signals;
use embassy_nrf::gpio::AnyPin;
use embassy_nrf::gpio::{Input, Pull};
use defmt::*;

const TASK_ID: &str = "SPEED";
const WHEEL_CIRCUMFERENCE: f32 = 1105.0; // 1105mm measured, (12.5inch diameter -> 317.5mm diameter -> 997.46mm circumference)
const SPEED_SMOOTH_FACTOR: f32 = 0.3;
const SENSOR_SEGMENTS: u8 = 15; // there are 15 segments on 1 revolution of the wheel. So 0-16 inclusive is a rotation. Then at 16, reset to 0.
const SEGMENT_LENGTH: f32 = 73.7; // mm (measured / 15)

#[embassy_executor::task]
pub async fn task(
    pin: AnyPin
) {
    info!("{}: start", TASK_ID);
    let pub_instant_speed = signals::INSTANT_SPEED.publisher().unwrap();
    let pub_smooth_speed = signals::SMOOTH_SPEED.publisher().unwrap();
    let pub_wheel_rotations = signals::WHEEL_ROTATIONS.publisher().unwrap();
    let pub_odometer = signals::ODOMETER.publisher().unwrap();

    let mut segment_count = 0;
    let mut rotation_count = 0;
    let mut last_rotation_time;
    let mut rotation_time = embassy_time::Instant::now().as_micros();
    let mut smooth_speed = 0.0;
    let mut pin_state = Input::new(pin, Pull::Down); // low

    loop {
        pin_state.wait_for_low().await; // not sure if we need this one?
        pin_state.wait_for_high().await; // pin switched from low to high
        
        // speed calculated 15x per rotation
        // this may be to much... but could be more reliable at low speeds?
        last_rotation_time = rotation_time;
        rotation_time = embassy_time::Instant::now().as_micros();
        let delta_time: f32 = (rotation_time - last_rotation_time) as f32;

        // false readings for very slow speeds, or too high speeds
        if delta_time > 20.0 && delta_time < 5000.0 {
            // mm per second -> kms (1mm/s = 0.0036km/s)
            let instant_speed: f32 = (1000.0 / delta_time) * SEGMENT_LENGTH * 0.0036;
            pub_instant_speed.publish_immediate(instant_speed as u32); // round

            // calculate smooth speed
            let delta_speed : f32 = instant_speed - smooth_speed; // calc difference btween speeds
            let speed_adjust = delta_speed * SPEED_SMOOTH_FACTOR; // todo: multiply by delta time, so faster speeds are adjusted faster?
            smooth_speed += speed_adjust;
            pub_smooth_speed.publish_immediate(smooth_speed as u8); // round
        }

        // odometer on full rotations
        // another segment completed
        segment_count += 1;
        if segment_count >= SENSOR_SEGMENTS {
            segment_count = 0;

            // increment wheel rotations
            rotation_count += 1;
            pub_wheel_rotations.publish_immediate(rotation_count);

            // odometer
            let odometer = (WHEEL_CIRCUMFERENCE * rotation_count as f32 * 0.0036) as u8;  // round mm to km
            pub_odometer.publish_immediate(odometer);
        };

    }
}