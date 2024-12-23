use crate::utils::signals;
use embassy_nrf::gpio::AnyPin;
use embassy_nrf::gpio::{Input, Pull};
use defmt::info;

const TASK_ID: &str = "SPEED";
const WHEEL_CIRCUMFERENCE: f32 = 1105.0; // 1105mm measured, (12.5inch diameter -> 317.5mm diameter -> 997.46mm circumference)
const SPEED_SMOOTH_FACTOR: f32 = 0.3;
const SENSOR_SEGMENTS: u16 = 1000; // there are 15 segments on 1 revolution of the wheel. So 0-16 inclusive is a rotation. Then at 16, reset to 0.
//const SEGMENT_LENGTH: f32 = 73.7; // mm (measured / 15)

#[embassy_executor::task]
pub async fn task(
    pin: AnyPin
) {
    info!("{}", TASK_ID);

    // debug
    return;

    let send_instant_speed = signals::INSTANT_SPEED.sender();
    let send_smooth_speed = signals::SMOOTH_SPEED.sender();
    let send_wheel_rotations = signals::WHEEL_ROTATIONS.sender();
    let send_odometer = signals::ODOMETER.sender();

    let mut segment_count = 0;
    let mut rotation_count = 0_u16;
    let mut last_time = embassy_time::Instant::now().as_micros();
    let mut smooth_speed = 0.0;
    let mut pin_state = Input::new(pin, Pull::Down); // low

    loop {
        pin_state.wait_for_low().await; // not sure if we need this one?
        pin_state.wait_for_high().await; // pin switched from low to high
      
        // another sensor segment completed
        segment_count += 1;


        // full rotation
        if segment_count > SENSOR_SEGMENTS {
            segment_count = 0;

            // increment wheel rotations
            rotation_count += 1;
            //send_wheel_rotations.send(rotation_count); //TODO: is this used?

            // odometer
            let odometer = (WHEEL_CIRCUMFERENCE * f32::from(rotation_count) / 1_000_f32 / 1_00_f32) as u16;  // round mm -> m -> km.m
            send_odometer.send(odometer); // TODO: fix

            info!("odometer : {}, {}", odometer, rotation_count);


            // calculate speed = dist / time

            //get time delta
            let current_time = embassy_time::Instant::now().as_micros();
            //let delta_time = current_time - last_time;
            let delta_time: f32 = (current_time - last_time) as f32; // microseconds
            last_time = current_time;


            let delta_time_seconds = delta_time / 1_000_000_f32; // micro to seconds
            let distance = WHEEL_CIRCUMFERENCE / 1_000_f32; // mm to m
            let instant_speed: f32 = (distance / delta_time_seconds) * 3.6; // m/sec to km/hr
            send_instant_speed.send(instant_speed as u32); // round

            // calculate smooth speed
            let delta_speed : f32 = instant_speed - smooth_speed; // calc difference btween speeds
            let speed_adjust = delta_speed * SPEED_SMOOTH_FACTOR; // todo: multiply by delta time, so faster speeds are adjusted faster?
            smooth_speed += speed_adjust;
            send_smooth_speed.send(smooth_speed as u8); // round

            info!("{} : {} : {}, {}", TASK_ID, instant_speed, smooth_speed, delta_time_seconds);
        
        };

    }
}