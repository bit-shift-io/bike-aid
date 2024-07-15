use crate::tune;

use embassy_nrf::gpio::{AnyPin, OutputDrive};
use defmt::*;
use embassy_nrf::peripherals::PWM0;
use embassy_nrf::pwm::{Prescaler, SimplePwm};
use embassy_time::Timer;

const TASK_ID: &str = "PIEZO";

#[embassy_executor::task]
pub async fn piezo (
    pwm_device: PWM0,
    pin: AnyPin
) {
    info!("{}: start", TASK_ID);

    let tempo = 300_u64; // ms
    let mut pwm = SimplePwm::new_1ch(pwm_device, pin);
    let duty = pwm.max_duty() / 10; // piezo driver suggests 50% of max duty

    // TODO: make function
    // https://circuitdigest.com/microcontroller-projects/playing-melodies-on-piezo-buzzer-using-arduino-tone-function

    let tune = tune::PIRATES_NOTE;
    let duration = tune::PIRATES_DURATION;

    loop {
        info!("loop start");

        let mut i = 0;
        for note in tune {
            // play note
            let time = 1000/duration[i] as u64;
            pwm.enable();
            let note = note.try_into().unwrap();
            if note !=0 {
                pwm.set_period(note); // TODO: try 16_000_000 / tone.1
                pwm.set_duty(0, duty); // duty changes with period, so needs to be reset each time
            }

            Timer::after_millis(time).await;

            // pause
            let pause = time * 1.05 as u64; //Here 1.05 is tempo, increase to play it slower
            pwm.disable();
            Timer::after_millis(pause).await;

            i+=1;
        }
    }
}