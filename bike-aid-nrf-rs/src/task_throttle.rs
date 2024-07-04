#![allow(non_snake_case)]

use crate::signals;
use crate::functions::*;
use defmt::*;

static TASK_ID : &str = "THROTTLE";

#[embassy_executor::task]
pub async fn throttle (
    //mut saadc: Saadc<'static, 1>,
    //pin_adc: AnyInput,
    //saadc: SAADC,
) {
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

    // supply voltage - 4.36v
    let MAP_IN_MIN = 199; // 0.847v no throttle
    let MAP_IN_MAX = 840; // 3.58v full throttle
    let MAP_OUT_MIN = 288; // 1.23v just before motor active
    let MAP_OUT_MAX = 1023; //620 // 2.6v just after max speed
    /* 
    Smoothing - Jerkiness Mitigation
    ===========================
    how quickly to adjust output, larger values are slower
    smoothing over time
    */
    let INCREASE_SMOOTH_FACTOR = 4000;
    let DECREASE_SMOOTH_FACTOR = 100;

    /* 
    Speed Limit
    ===========================
    adjusts throttle output speed limit
    */
    let LIMIT_MAP_OUT_MIN = 100;
    let LIMIT_MAP_OUT_MAX = 1023;


    let pub_throttle = signals::THROTTLE_OUT.publisher().unwrap();
    let mut sub_throttle = signals::THROTTLE_IN.subscriber().unwrap();
    let mut output = 0;

    info!("{} : Entering main loop", TASK_ID);
    loop {
        let input = sub_throttle.next_message_pure().await;

        //info!("{}", input);

        // clamp to positive values only
        //let input = clamp_positive(input);

        // Debug: convert to voltage
        // ADC - 6.144 / 32768 = 0.0001875V (15 bit)
        let voltage = (f32::from(input) * 0.1875) as u16; // converted to mv

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
        if output > map(limit_input, &0, &1023, &LIMIT_MAP_OUT_MIN, &LIMIT_MAP_OUT_MAX) {
            adjustment = min(adjustment, 0); // always allow decrease
        }

        output += adjustment;

        //throttle to output value map - mapping to controller range
        let mapped_output = map(output, &MAP_IN_MIN, &MAP_IN_MAX, &MAP_OUT_MIN, &MAP_OUT_MAX);

        // DAC 0 - 4095 output - 12 bit

        // TODO:mapped_output values are 0-1023 for arduino, what do we use on the dac??
        // TODO: check if these can be negative values, the dac only takes positive values

        pub_throttle.publish_immediate(mapped_output); 
        info!("in:{} | out: {} | map: {} | mv: {}", input, output, mapped_output, voltage);
    }
}