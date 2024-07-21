use crate::store;
use crate::signals;
use crate::functions::*;
use defmt::*;

const TASK_ID: &str = "THROTTLE";

#[embassy_executor::task]
pub async fn throttle () {
    info!("{}: start", TASK_ID);
  
    let pub_throttle = signals::THROTTLE_OUT.publisher().unwrap();
    let mut sub_throttle = signals::THROTTLE_IN.subscriber().unwrap();
    let mut sub_instant_speed = signals::INSTANT_SPEED.subscriber().unwrap();
    let mut output = 0;

    loop {
        let input = sub_throttle.next_message_pure().await; // millivolts

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
        // as we get closer to the desired speed, we decrease the adjustment
        // apply speed limit - allow increase  only if bellow limit
        let speed_limit = throttle_settings.speed_limit; // in kmhr
        if speed_limit > 0 {
            // get current speed
            // might need assert_eq!(sub0.try_next_message(), None);
            let instant_speed = sub_instant_speed.try_next_message(); // poll

            // as instant speed appraches current speed, we decrease the adjustment
            if output > map(limit_input, &0, &instant_speed, &0, &speed_limit) {
                adjustment = min(adjustment, 0); // always allow decrease
            }

            output += adjustment;
        }

        // old method uses a min and max number
        /*
        let limit_input = 1023;

        // apply speed limit - allow increase  only if bellow limit
        if output > map(limit_input, &0, &1023, &throttle_settings.limit_min, &throttle_settings.limit_max) {
            adjustment = min(adjustment, 0); // always allow decrease
        }

        output += adjustment;
         */


        // deadband/deadzone map
        // throttle to output value map - mapping to controller range
        let mapped_output = map(output, &throttle_settings.deadband_in_min, &throttle_settings.deadband_in_max, &throttle_settings.deadband_out_min, &throttle_settings.deadband_out_max);

        // TODO: check if these can be negative values, the dac only takes positive values
        // TODO: check we are converting to mv out. I think we are!

        pub_throttle.publish_immediate(mapped_output); 
        info!("mv_in:{} | out: {} | map: {}", input, output, mapped_output);
    }
}