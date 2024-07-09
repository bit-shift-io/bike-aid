#![allow(non_snake_case)]

use crate::store;
use crate::signals;
use crate::functions::*;
use defmt::*;

const TASK_ID: &str = "THROTTLE";

#[embassy_executor::task]
pub async fn throttle () {
    info!("{}: start", TASK_ID);
    /* 
    Deadband / Deadzone
    ===========================
    Adjust throttle range to eliminate deadband/deadzones

    MAP_IN - Normal range of throttle
    MAP_OUT - range to output to controller

    All the ranges below can be determined by watching the serial console and twisting the throttle, they will be slightly wrong if the controller supplies less than 5v USB to throttle.
    Preferably, use a multimeter to measure voltage output from the throttle on your ebike and use the formula like so to calculate the numbers:
    ( Signal Voltage / Supply Voltage ) * 1023

    MAP_IN_MIN - Voltage when the throttle is unpressed
    MAP_IN_MAX - Voltage when the throttle is fully pressed
    MAP_OUT_MIN - Voltage just before the motor starts to activate the wheels
    MAP_OUT_MAX - Voltage just after max speed (or use supply voltage otherwise)

    Then verify the output with a multimeter also to tweak the ranges MAP_OUT_MIN, and MAP_OUT_MAX
    */
    let DEADBAND_IN_MIN = store::THROTTLE_DEADBAND_IN_MIN.lock().await.clone();
    let DEADBAND_IN_MAX = store::THROTTLE_DEADBAND_IN_MAX.lock().await.clone();
    let DEADBAND_OUT_MIN = store::THROTTLE_DEADBAND_OUT_MIN.lock().await.clone();
    let DEADBAND_OUT_MAX = store::THROTTLE_DEADBAND_OUT_MAX.lock().await.clone();

    /* 
    Smoothing - Jerkiness Mitigation
    ===========================
    how quickly to adjust output, larger values are slower
    smoothing over time
    */
    let INCREASE_SMOOTH_FACTOR = store::THROTTLE_INCREASE_SMOOTH_FACTOR.lock().await.clone();
    let DECREASE_SMOOTH_FACTOR = store::THROTTLE_DECREASE_SMOOTH_FACTOR.lock().await.clone();

    /* 
    Speed Limit
    ===========================
    adjusts throttle output speed limit
    */
    let LIMIT_MIN = store::THROTTLE_LIMIT_MIN.lock().await.clone();
    let LIMIT_MAX = store::THROTTLE_LIMIT_MAX.lock().await.clone();

    // disable throttle mapping, deadband etc. Pass input to output
    let PASSTHROUGH = store::THROTTLE_PASSTHROUGH.lock().await.clone();


    let pub_throttle = signals::THROTTLE_OUT.publisher().unwrap();
    let mut sub_throttle = signals::THROTTLE_IN.subscriber().unwrap();
    let mut output = 0;

    loop {
        let input = sub_throttle.next_message_pure().await; // millivolts

        if PASSTHROUGH {
            info!("mv: {} ", input);
            pub_throttle.publish_immediate(input);
            continue;
        }
        
        // delta computer from last output value
        let delta = input - output;

        // how much to change throttle this itteration (+/-)
        let mut adjustment = match delta {
            d if d > 0 => d / INCREASE_SMOOTH_FACTOR,
            _ => delta / DECREASE_SMOOTH_FACTOR,
        };


        // speed limiter
        // TODO: make based on speed option
        let limit_input = 1023;

        // apply speed limit - allow increase  only if bellow limit
        if output > map(limit_input, &0, &1023, &LIMIT_MIN, &LIMIT_MAX) {
            adjustment = min(adjustment, 0); // always allow decrease
        }

        output += adjustment;

        // throttle to output value map - mapping to controller range
        let mapped_output = map(output, &DEADBAND_IN_MIN, &DEADBAND_IN_MAX, &DEADBAND_OUT_MIN, &DEADBAND_OUT_MAX);

        // DAC 0 - 4095 output - 12 bit

        // TODO:mapped_output values are 0-1023 for arduino, what do we use on the dac??
        // TODO: check if these can be negative values, the dac only takes positive values

        pub_throttle.publish_immediate(mapped_output); 
        info!("mv_in:{} | out: {} | map: {}", input, output, mapped_output);
    }
}