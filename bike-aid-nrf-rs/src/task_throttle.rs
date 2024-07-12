use crate::store;
use crate::signals;
use crate::functions::*;
use defmt::*;
use embassy_time::Duration;
use embassy_time::Timer;

const TASK_ID: &str = "THROTTLE";

#[embassy_executor::task]
pub async fn throttle () {
    info!("{}: start", TASK_ID);
  
    let pub_throttle = signals::THROTTLE_OUT.publisher().unwrap();
    let mut sub_throttle = signals::THROTTLE_IN.subscriber().unwrap();
    let mut output = 0;

    loop {
        // for testing without messages
        //Timer::after(Duration::from_millis(100)).await; 
        //let input = 100;

        let input = sub_throttle.next_message_pure().await; // millivolts

        // TODO: we could use a polled channel here instead of a mutex... Which is better?
        let throttle_settings = store::THROTTLE_SETTINGS.lock().await;

        if throttle_settings.passthrough {
            info!("mv: {} ", input);
            pub_throttle.publish_immediate(input);
            continue;
        }
        
        // delta computer from last output value
        let delta = input - output;

        // how much to change throttle this itteration (+/-)
        let mut adjustment = match delta {
            d if d > 0 => d / throttle_settings.increase_smooth_factor,
            _ => delta / throttle_settings.decrease_smooth_factor,
        };


        // speed limiter
        // TODO: make based on speed option
        let limit_input = 1023;

        // apply speed limit - allow increase  only if bellow limit
        if output > map(limit_input, &0, &1023, &throttle_settings.limit_min, &throttle_settings.limit_max) {
            adjustment = min(adjustment, 0); // always allow decrease
        }

        output += adjustment;

        // throttle to output value map - mapping to controller range
        let mapped_output = map(output, &throttle_settings.deadband_in_min, &throttle_settings.deadband_in_max, &throttle_settings.deadband_out_min, &throttle_settings.deadband_out_max);

        // DAC 0 - 4095 output - 12 bit

        // TODO:mapped_output values are 0-1023 for arduino, what do we use on the dac??
        // TODO: check if these can be negative values, the dac only takes positive values

        pub_throttle.publish_immediate(mapped_output); 
        info!("mv_in:{} | out: {} | map: {}", input, output, mapped_output);
    }
}