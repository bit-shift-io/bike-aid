use crate::utils::functions;
use crate::utils::store;
use crate::utils::signals;
use defmt::*;
use embassy_futures::select::{select, Either};
use num_traits::Pow;
use num_traits::Float;

const TASK_ID: &str = "THROTTLE";

#[embassy_executor::task]
pub async fn task() {
    info!("{}: start", TASK_ID);
    
    let mut sub_power = signals::SWITCH_POWER.subscriber().unwrap();
    let mut power_state = false;

    loop { 
        if let Some(b) = sub_power.try_next_message_pure() {power_state = b}
        match power_state {
            true => {
                let power_future = sub_power.next_message_pure();
                let task_future = run();
                match select(power_future, task_future).await {
                    Either::First(val) => { power_state = val; }
                    Either::Second(_) => {} // other task will never end
                }
            },
            false => { power_state = sub_power.next_message_pure().await; }
        }
    }
}


async fn run() {
    let pub_throttle = signals::THROTTLE_OUT.publisher().unwrap();
    let mut sub_throttle = signals::THROTTLE_IN.subscriber().unwrap();
    //let sub_instant_speed = signals::INSTANT_SPEED.subscriber().unwrap();
    let mut output_voltage = 0.0;
    let mut input_history = InputHistory::new();

    loop {
        // we are converting to f32 as we have divide issues with i16
        let input_voltage = sub_throttle.next_message_pure().await as f32; // millivolts

        let throttle_settings = store::THROTTLE_SETTINGS.lock().await;

        // direct pass through for debug or pure fun off road!
        if throttle_settings.passthrough {
            //info!("{}: passthrough mv: {} ", TASK_ID, input_voltage);
            pub_throttle.publish_immediate(input_voltage as i16);
            continue;
        }
        
        // moving averages smoothing
        let input_smooth = input_history.add(input_voltage);

        // delta computer from last output value
        let delta = input_smooth - output_voltage;


        // let use linear steps to control smoothing
        // we can then use u16 values instead of f32
        let mut adjustment;
        if delta > 0.0 { // increase speed
            adjustment = throttle_settings.increase_smooth_factor as f32;
            // cap step so we dont go over
            if adjustment + output_voltage > input_smooth {
                adjustment = input_smooth - output_voltage;
            }
        } else { // decrease speed
            adjustment = -(throttle_settings.decrease_smooth_factor as f32);
            // cap step so we dont go under
            if adjustment + output_voltage < input_smooth {
                adjustment = input_smooth - output_voltage;
            }
        }


        /*
        old smoothing method
        // how much to change throttle this itteration (+/-)
        // use smoothing factor as scale
        // TODO: clamp to 0-100 range smoothing settings
        // TODO: to ensure multiplier is never 0 or over 1
        let multiplier;
        if delta > 0.0 {
            multiplier = (100.0 - (throttle_settings.increase_smooth_factor as f32)) * SMOOTHING_MULTIPLIER; 
        } else {
            multiplier = (100.0 - (throttle_settings.decrease_smooth_factor as f32)) * SMOOTHING_MULTIPLIER;
        }
        let mut adjustment = delta * multiplier;
         */


         /*
        // old speed limiter
        // this could go at the end of this code and map to the range of the deadband?

        // apply speed limit - allow increase  only if bellow limit
        // if output_voltage is larger than speed limit... set adjustment to 0
        if output_voltage > (throttle_settings.speed_limit as f32) {
            adjustment = functions::min(adjustment, 0.0); // always allow decrease
        }
         */

        // apply adjustment/step
        output_voltage += adjustment;

        

        // how to do speed based limit:
        // As we approach speed limit, adjust deadband_out_max to match the current MV value! This should give speed based limit
        // if we are at max mv, and speed is not enough, increase till we are back in range.
        // if we are over mv and speed it to high, reduce it...

        // deadband/deadzone map
        // throttle to output value map - mapping to controller range
        let mapped_output = functions::map(output_voltage, &(throttle_settings.throttle_min as f32), &(throttle_settings.throttle_max as f32), &(throttle_settings.deadband_min as f32), &(throttle_settings.deadband_max as f32));

        // TODO: check if these can be negative values, the dac only takes positive values

        pub_throttle.publish_immediate(mapped_output as i16); 
        info!("throttle: {} | out: {} | map: {}  -  delta: {} | adj: {}", input_smooth as i16, output_voltage as i16, mapped_output as i16, delta, adjustment);

        // DEBUG: remove this as it will clog up the ble connection
        // publish to uart for debug
        // let mut buf = [0u8; 32];
        // let text = format_no_std::show(&mut buf, format_args!("{},{}\n", input_smooth, output_voltage)).unwrap();
        // let s = String::try_from(text).unwrap();
        // signals::UART_WRITE.dyn_immediate_publisher().publish_immediate(s);
    }
}


// function for applying throttle curve
// we can make the lower values of the range easier to use on the throttle
// exponent of 1 is linear, while 0.3 will increase the lower range of values
// this will output a 0-1 value
fn apply_throttle_curve(input_value: i32, min_input: i32, max_input: i32, min_output: i32, max_output: i32, exponent: f32) -> i32 {
    // Normalize the input value to the range [0, 1]
    let normalized_value = (input_value - min_input) as f32 / (max_input - min_input) as f32;
    
    // Apply the curve function (e.g., power function)
    let curved_value = normalized_value.pow(exponent);
    
    // Map back to the output range
    let output_value = min_output + (curved_value * (max_output - min_output) as f32).round() as i32;
    
    output_value
}


// a helper class to keep a track of smoothing
struct InputHistory {
    data: [f32; 3],
    index: usize,
}

impl InputHistory {
    fn new() -> Self {
        InputHistory { 
            data: [0.0; 3],
            index: 0,
        }
    }

    fn add(&mut self, value: f32) -> f32 {
        // add to current index
        self.data[self.index] = value;

        let length = self.data.len();

        // increase index, wrap around if larger than size
        self.index = (self.index + 1) % length;

        // Calculate the average if we have at least 5 elements
        let sum: f32 = self.data.iter().sum();
        sum / length as f32 // Return the average
    }

}