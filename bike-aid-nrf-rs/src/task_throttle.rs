/*
For NRF52840
Analog pin  GPIO pin
AIN0        P0.02
AIN1        P0.03
AIN2        P0.04
AIN3        P0.05
AIN4        P0.28
AIN5        P0.29
AIN6        P0.30
AIN7        P0.31
*/

use crate::signals;
use crate::functions::*;
use embassy_nrf::peripherals::SAADC;
use embassy_nrf::saadc::AnyInput;
use embassy_nrf::saadc::{ChannelConfig, Config, Saadc};
use embassy_nrf::{bind_interrupts, saadc};
use embassy_time::Timer;
use defmt::*;

static TASK_ID : &str = "THROTTLE";

#[embassy_executor::task]
pub async fn throttle (
    //mut saadc: Saadc<'static, 1>,
    pin_adc: AnyInput,
    saadc: SAADC,
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

    /* 
    ===========================
    */

    bind_interrupts!(struct Irqs {
        SAADC => saadc::InterruptHandler;
    });
    let config = Config::default(); // default 12 bit, bypass no pull resistors, gain 1/6, reference internal
    let channel_config = ChannelConfig::single_ended(pin_adc);
    let mut adc = Saadc::new(saadc, Irqs, config, [channel_config]);
    adc.calibrate().await; // calibrate
    Timer::after_millis(500).await;
    let pub_throttle = signals::THROTTLE.publisher().unwrap();
    let mut output = 0;

    info!("{} : Entering main loop", TASK_ID);
    loop {
        // defaults:
        // 12 bit
        // bypass no pull resistors
        // gain 1/6
        // reference internal (0.6v)
        // Input range = (0.6 V)/(1/6) = 3.6 V

        // the following formula is used by the chip
        // RESULT = [V(P) - V(N)] * GAIN / REFERENCE * (2 ^ (RESOLUTION - m))
        // or
        // V(P) - V(N) = RESULT * REFERENCE / GAIN / (2 ^ (RESOLUTION - m))
        //
        // Result = sample/reading from saadc
        // Voltage positive (P) = 3.6 V
        // Voltage negative (N) = 0 V (single ended)
        // Gain = 1/6
        // Reference = 0.6 V
        // Resolution = 12
        // m = 0 (single ended) or 1 (differental mode)

        let mut buf = [0; 1];
        adc.sample(&mut buf).await;
        let input = buf[0];

        info!("{}", input);

        // clamp to positive values only
        let input = clamp_positive(input);

        // Debug: convert to voltage
        let voltage = f32::from(input) * 3600.0 / 4096.0; // converted to mv

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

        // throttle to output value map - mapping to controller range
        let mapped_output = map(output, &MAP_IN_MIN, &MAP_IN_MAX, &MAP_OUT_MIN, &MAP_OUT_MAX);

        // DAC 0 - 4095 output - 12 bit
        // SAADC -2048 to 2047 input - 12 bit

        // TODO:mapped_output values are 0-1023 for arduino, what do we use on the dac??
        // TODO: check if these can be negative values, the dac only takes positive values

        //pub_throttle.publish_immediate(mapped_output); 
        info!("mv: {} |in:{} | out: {} | map: {}", voltage, input, output, mapped_output);

        Timer::after_millis(100).await;
    }
}


/*
Note: The old method was to create this in main, then pass the whole value into the task.
We may need to do this in the future if we have multiple adc in use??

    /*
    let saadc = {
        use embassy_nrf::saadc::{ChannelConfig, Config, Saadc};
        use embassy_nrf::{bind_interrupts, saadc};
        bind_interrupts!(struct Irqs {
            SAADC => saadc::InterruptHandler;
        });
        let config = Config::default(); // default 12 bit, bypass no pull resistors, gain 1/6, reference internal
        let mut pin = p.P0_31;
        let channel_config = ChannelConfig::single_ended(&mut pin);
        Saadc::new(p.SAADC, Irqs, config, [channel_config])
    };
     */

*/