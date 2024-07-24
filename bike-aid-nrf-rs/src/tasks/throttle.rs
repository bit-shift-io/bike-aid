use crate::utils::functions;
use crate::utils::store;
use crate::utils::signals;
use defmt::*;
use heapless::String;

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

        if throttle_settings.passthrough {
            info!("{}: passthrough mv: {} ", TASK_ID, input_voltage);
            pub_throttle.publish_immediate(input_voltage);
            continue;
        }
        
        // delta computer from last output value
        let delta = input_voltage - output_voltage;

        // how much to change throttle this itteration (+/-)
        // use smoothing factor as scale
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

        // apply speed limit - allow increase  only if bellow limit
        // if output_voltage is larger than speed limit... set adjustment to 0
        if output_voltage > throttle_settings.speed_limit {
            adjustment = functions::min(adjustment, 0); // always allow decrease
        }

        output_voltage += adjustment;


        

        // how to do speed based limit:
        // As we approach speed limit, adjust deadband_out_max to match the current MV value! This should give speed based limit
        // if we are at max mv, and speed is not enough, increase till we are back in range.
        // if we are over mv and speed it to high, reduce it...

        // deadband/deadzone map
        // throttle to output value map - mapping to controller range
        let mapped_output = functions::map(output_voltage, &throttle_settings.no_throttle, &throttle_settings.full_throttle, &throttle_settings.deadband_min, &throttle_settings.deadband_max);

        // TODO: check if these can be negative values, the dac only takes positive values

        pub_throttle.publish_immediate(mapped_output); 
        info!("mv_in:{} | out: {} | map: {}", input_voltage, output_voltage, mapped_output);

        // publish to uart for debug
        let mut buf = [0u8; 32];
        let text = format_no_std::show(&mut buf, format_args!("{},{},{}\n", input_voltage, output_voltage, mapped_output)).unwrap();
        let s = String::try_from(text).unwrap();
        signals::UART_WRITE.dyn_immediate_publisher().publish_immediate(s);
        info!("{}: debug string: {}", TASK_ID, text);
    }
}