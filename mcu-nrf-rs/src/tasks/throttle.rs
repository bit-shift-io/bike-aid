use crate::utils::functions;
use crate::utils::signals;
use defmt::*;
use num_traits::Float;

const TASK_ID: &str = "THROTTLE";
const SPEED_STEP: u16 = 1200;

#[embassy_executor::task]
pub async fn task() {
    info!("{}: start", TASK_ID);
    
    let pub_throttle = signals::THROTTLE_OUT.publisher().unwrap();
    let mut sub_throttle = signals::THROTTLE_IN.subscriber().unwrap();
    let mut output_voltage = 0u16;
    let throttle_settings = signals::THROTTLE_SETTINGS.lock().await.clone();

    loop {
        let throttle_voltage = sub_throttle.next_message_pure().await; // millivolts

        // direct pass through for debug or pure fun off road!
        if throttle_settings.passthrough {
            //info!("{}: passthrough mv: {} ", TASK_ID, input_voltage);
            pub_throttle.publish_immediate(throttle_voltage);
            continue;
        }

        // get throttle voltage
        let mut input_voltage = throttle_voltage;
        
        // get mutex values, minimise lock time
        let (cruise_voltage, brake_on) = {
            let cruise_voltage = *signals::CRUISE_VOLTAGE.lock().await;
            let brake_on = *signals::BRAKE_ON_MUTEX.lock().await;
            (cruise_voltage, brake_on)
        };
    
        // check brake & cruise conditions
        if brake_on { 
            input_voltage = throttle_settings.throttle_min; // use min
        } else if input_voltage < cruise_voltage {
            input_voltage = cruise_voltage; // if throttle bellow cruise, use cruise
        }
        
        // smoothing
        output_voltage = smooth(input_voltage, output_voltage, throttle_settings.increase_smooth_factor, throttle_settings.decrease_smooth_factor).await;

        // minimum speed step if throttle is above threshold
        if throttle_voltage > SPEED_STEP && output_voltage < SPEED_STEP {
            //info!("speed step");
            output_voltage = SPEED_STEP;
        }

        // how to do speed based limit:
        // As we approach speed limit, adjust deadband_out_max to match the current MV value! This should give speed based limit
        // if we are at max mv, and speed is not enough, increase till we are back in range.
        // if we are over mv and speed it to high, reduce it...

        // deadband/deadzone map
        // throttle to output value map - mapping to controller range
        //info!("{} | {} -> {} {} -> {} {}", throttle_voltage, output_voltage, throttle_settings.throttle_min, throttle_settings.throttle_max, throttle_settings.deadband_min, throttle_settings.deadband_max);
        let mapped_output = functions::map(output_voltage, throttle_settings.throttle_min, throttle_settings.throttle_max, throttle_settings.deadband_min, throttle_settings.deadband_max);
        pub_throttle.publish_immediate(mapped_output); 
        //info!("throttle: {} | out: {} | map: {}", throttle_voltage, output_voltage, mapped_output);
    }
}


async fn smooth(
    input_voltage: u16, 
    output_voltage: u16, 
    increase_smooth_factor: u16, 
    decrease_smooth_factor: u16
) -> u16 {
    let delta = input_voltage as i16 - output_voltage as i16;
    let mut adjustment = 0i16;

    if delta > 0 { // increase speed
        adjustment = increase_smooth_factor as i16;
        // cap step so we dont go over
        if (adjustment + output_voltage as i16) > (input_voltage as i16) {
            adjustment = input_voltage as i16 - output_voltage as i16;
        }
    } else if delta < 0 { // decrease speed
        adjustment = -(decrease_smooth_factor as i16);
        // cap step so we dont go under
        if (adjustment + output_voltage as i16) < (input_voltage as i16) {
            adjustment = input_voltage as i16 - output_voltage as i16;
        }
    }

    //info!("delta {} | adj {}", delta, adjustment);
    // Apply the adjustment to the output voltage
    let result_voltage = (output_voltage as i16 + adjustment) as u16;
    result_voltage
}


fn throttle_curve(
    input_value: u16,
    min_input: u16,
    max_input: u16,
    min_output: u16,
    max_output: u16,
    exponent: f32,
) -> u16 {
    // Ensure input values are within the specified range
    if input_value < min_input || input_value > max_input {
        return if input_value < min_input { min_output } else { max_output };
    }

    // Normalize the input value to the range [0, 1]
    let normalized_value = (input_value - min_input) as f32 / (max_input - min_input) as f32;

    // Apply the curve function (e.g., power function)
    let curved_value = normalized_value.powf(exponent);

    // Map back to the output range
    let output_value = min_output as f32 + (curved_value * (max_output - min_output) as f32).round();

    // Ensure the output value is within the u16 range
    output_value.clamp(0.0, u16::MAX as f32) as u16
}