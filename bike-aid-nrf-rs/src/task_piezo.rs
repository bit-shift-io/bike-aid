use crate::utils::melody;

use embassy_nrf::gpio::AnyPin;
use defmt::*;
use embassy_nrf::peripherals::PWM0;
use embassy_nrf::pwm::SimplePwm;
use embassy_time::Timer;

const TASK_ID: &str = "PIEZO";

#[embassy_executor::task]
pub async fn piezo (
    pwm_device: PWM0,
    pin: AnyPin
) {
    info!("{}: start", TASK_ID);

    let mut pwm = SimplePwm::new_1ch(pwm_device, pin);
    let duty = pwm.max_duty() / 10; // piezo driver suggests 50% of max duty
    let tune = melody::SUPER_MARIO_BROS;
    let tempo = melody::SUPER_MARIO_TEMPO; // beats per minute
    let length = tune.len() / 2;
    let wholenote = (60000.0 * 4.0) / tempo as f32; // wholenote (ms) = 60,000 (1 minute in ms) * 4 (length of whole note) / tempo (bpm)
    
    loop {
        // loop over each note,duration combo
        for n in (0..=length).step_by(2) {

            // calculates the duration of each note
            let mut duration: f32 = 0.0;
            let divider = tune[n + 1] as i16; // can be negative
            if divider > 0 {
                // regular note, just proceed
                duration = wholenote / divider as f32;
            } else if divider < 0 {
                // dotted notes are represented with negative durations!!
                duration = wholenote / divider.abs() as f32;
                duration *= 1.5 as f32; // increases the duration in half for dotted notes
            }
            let note_duration = (duration * 0.9) as u64;
            let delay_duration = (duration * 0.1) as u64;

            // get the note value
            let note_val = tune[n].try_into().unwrap();
            if note_val != 0 { // 0 is a rest, so no tune
                pwm.enable();
                pwm.set_period(note_val); // TODO: try 16_000_000 / tone.1
                pwm.set_duty(0, duty); // duty changes with period, so needs to be reset each time
            }

            // Wait for the specief duration before playing the next note.
            // we only play the note for 90% of the duration, leaving 10% as a pause
            Timer::after_millis(note_duration).await;

            // stop the waveform generation before the next note.
            pwm.disable();
            Timer::after_millis(delay_duration).await;
        }
    }
}