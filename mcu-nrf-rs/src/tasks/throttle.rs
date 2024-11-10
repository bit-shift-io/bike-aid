use crate::utils::functions;
use crate::utils::settings;
use crate::utils::signals;
use defmt::info;

const TASK_ID: &str = "THROTTLE";

#[embassy_executor::task]
pub async fn task() {
    info!("{}", TASK_ID);
    
    let mut rec_throttle_settings = settings::THROTTLE_SETTINGS.receiver().unwrap();
    let mut rec_cruise_voltage = settings::CRUISE_VOLTAGE.receiver().unwrap();
    let send_throttle = signals::THROTTLE_OUT.sender();
    let mut rec_throttle = signals::THROTTLE_IN.receiver().unwrap();
    let mut output_voltage = 0u16;
    let mut rec_brake_on = signals::BRAKE_ON.receiver().unwrap();
    let mut count = 0u8;
    let mut last_voltage = 0u16;

    loop {
        let throttle_voltage = rec_throttle.changed().await; // millivolts

        let settings = rec_throttle_settings.try_get().unwrap();
        if settings.passthrough {
            send_throttle.send(throttle_voltage);
            continue;
        }

        // get values
        let cruise_voltage = rec_cruise_voltage.try_get().unwrap();
        let brake_on = rec_brake_on.try_get().unwrap();
        let mut target_voltage = throttle_voltage;

        // check brake, cruise and target conditions
        if brake_on { target_voltage = settings.throttle_min; } // throttle 0%
        else if target_voltage < cruise_voltage { target_voltage = cruise_voltage; } // if throttle bellow cruise, use cruise as target
        else if target_voltage < settings.throttle_min { target_voltage = settings.throttle_min; }; // if throttle bellow throttle min, use throttle min as target
        
        // ensure output_voltage is not under throttle min
        if output_voltage < settings.throttle_min { output_voltage = settings.throttle_min; }

        // if target is larger than the initial speed step, start at the speed step
        if (target_voltage > settings.speed_step || cruise_voltage > settings.speed_step) && output_voltage < settings.speed_step {
            output_voltage = settings.speed_step;
        }

        // smoothing
        output_voltage = smooth(target_voltage, output_voltage, settings);
        //info!("thr: {} -> tgt: {} -> out: {}", throttle_voltage, target_voltage, output_voltage);
        

/*
        // shouldnt need this anymore?
        if throttle_voltage < settings.speed_step && output_voltage < settings.speed_step && cruise_voltage == 0 { 
            // no throttle till hit threshold
            // this is to overcome the issue with the increasing voltage on the throttle line from the controller
            output_voltage = settings.throttle_min; // throttle 0%
        }
  */

        // how to do speed based limit:
        // As we approach speed limit, adjust deadband_out_max to match the current MV value! This should give speed based limit
        // if we are at max mv, and speed is not enough, increase till we are back in range.
        // if we are over mv and speed it to high, reduce it...

        // deadband/deadzone map
        // throttle to output value map - mapping to controller range
        //info!("{} | {} -> {} {} -> {} {}", throttle_voltage, output_voltage, throttle_settings.throttle_min, throttle_settings.throttle_max, throttle_settings.deadband_min, throttle_settings.deadband_max);
        let mapped_output = functions::map(output_voltage, settings.throttle_min, settings.throttle_max, settings.deadband_min, settings.deadband_max);
        send_throttle.send(mapped_output); 
        //info!("throttle: {} | out: {} | map: {}", throttle_voltage, output_voltage, mapped_output);

        
        if count >= 5 {
            count = 0;
            if output_voltage != last_voltage {
                last_voltage = output_voltage;
                // lower update for ble, every 500ms
                signals::send_ble(signals::BleHandles::ThrottleLevel, output_voltage.to_le_bytes().as_slice());
            };
        }
        count += 1;
    }
}


fn smooth(target_voltage: u16, output_voltage: u16, settings: settings::ThrottleSettings) -> u16 {
    let delta = target_voltage as i16 - output_voltage as i16;
    let mut adjustment = 0i16;

    if delta > 0 { // increase speed
        //adjustment = throttle_settings.increase_smoothing_low as i16;
        adjustment = get_smoothing_value(output_voltage as i16, settings);
        // cap step so we dont go over
        if (adjustment + output_voltage as i16) > (target_voltage as i16) {
            adjustment = target_voltage as i16 - output_voltage as i16;
        }
    } else if delta < 0 { // decrease speed
        adjustment = -(settings.decrease_smoothing as i16);
        // cap step so we dont go under
        if (adjustment + output_voltage as i16) < (target_voltage as i16) {
            adjustment = target_voltage as i16 - output_voltage as i16;
        }
    }

    //info!("{}: delta {} | adj {}", TASK_ID, delta, adjustment);
    // Apply the adjustment to the output voltage
    let result_voltage = (output_voltage as i16 + adjustment) as u16;
    result_voltage
}


fn get_smoothing_value(voltage: i16, settings: settings::ThrottleSettings) -> i16 {
    // prevent overflow with i32
    let normalized_value = (voltage as i32 - settings.throttle_min as i32) * (settings.increase_smoothing_high as i32 - settings.increase_smoothing_low as i32) / (settings.throttle_max as i32 - settings.throttle_min as i32);
    let result = settings.increase_smoothing_low as i16 + normalized_value as i16;
    //info!("{}: v {} res {} norm {}", TASK_ID, voltage, result, normalized_value as i16);
    return result;
}