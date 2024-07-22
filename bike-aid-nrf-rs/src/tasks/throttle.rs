use crate::utils::store;
use crate::utils::signals;
use crate::utils::functions::*;
use defmt::*;

const TASK_ID: &str = "THROTTLE";

#[embassy_executor::task]
pub async fn throttle () {
    info!("{}: start", TASK_ID);
  
    let pub_throttle = signals::THROTTLE_OUT.publisher().unwrap();
    let mut sub_throttle = signals::THROTTLE_IN.subscriber().unwrap();
    let mut sub_instant_speed = signals::INSTANT_SPEED.subscriber().unwrap();
    let mut output_voltage = 0;
    let mut last_delta_speed = 0.0;

    loop {
        let input_voltage = sub_throttle.next_message_pure().await; // millivolts

        let throttle_settings = store::THROTTLE_SETTINGS.lock().await;

        info!("{}", throttle_settings.passthrough);

        if throttle_settings.passthrough {
            info!("mv: {} ", input_voltage);
            pub_throttle.publish_immediate(input_voltage);
            continue;
        }
        
        // delta computer from last output value
        let delta = input_voltage - output_voltage;

        // how much to change throttle this itteration (+/-)
        let mut adjustment = match delta {
            d if d > 0 => d / throttle_settings.increase_smooth_factor,
            _ => delta / throttle_settings.decrease_smooth_factor,
        };



        /*
        // TODO: make based on speed option
        // as we get closer to the desired speed, we decrease the adjustment
        // apply speed limit - allow increase  only if bellow limit
        let speed_limit = throttle_settings.speed_limit; // in kmhr
        if speed_limit > 0 {
            // get current speed
            // might need assert_eq!(sub0.try_next_message(), None);
            let instant_speed = sub_instant_speed.try_next_message(); // poll
            let instant_speed = 15;

            let delta_scale = 3;
            let delta_speed = (speed_limit - instant_speed) * delta_scale;
            if delta_speed > 0 {
                adjustment = min(adjustment, delta_speed);
            }

            output_voltage += adjustment;
        }
         */

        // speed limiter
        // this could go at the end of this code and map to the range of the deadband?
        // this method uses a min and max number
        let limit_input = 1023; // this was pot value, should be a setting

        // apply speed limit - allow increase  only if bellow limit
        // if output_voltage is larger than speed limit... set adjustment to 0
        // TODO: limit_min and limit_max were values for the trimpot. We dont need these with ble as we can just specify a limit
        // so possibly change this to a percentage of max?
        if output_voltage > map(limit_input, &0, &1023, &throttle_settings.limit_min, &throttle_settings.limit_max) {
            adjustment = min(adjustment, 0); // always allow decrease
        }

        output_voltage += adjustment;


        

        // deadband/deadzone map
        // throttle to output value map - mapping to controller range
        let mapped_output = map(output_voltage, &throttle_settings.deadband_in_min, &throttle_settings.deadband_in_max, &throttle_settings.deadband_out_min, &throttle_settings.deadband_out_max);

        // TODO: check if these can be negative values, the dac only takes positive values
        // TODO: check we are converting to mv out. I think we are!

        pub_throttle.publish_immediate(mapped_output); 
        info!("mv_in:{} | out: {} | map: {}", input_voltage, output_voltage, mapped_output);
    }
}