use crate::signals;
use embassy_nrf::gpio::{AnyPin, OutputDrive};
use defmt::*;
use embassy_nrf::peripherals::PWM0;
use embassy_nrf::pwm::{Prescaler, SimplePwm};
use embassy_time::Timer;

const TASK_ID: &str = "PIEZO";

// https://dev.to/theembeddedrustacean/stm32f4-embedded-rust-at-the-hal-pwm-buzzer-3f1b
// https://dev.to/theembeddedrustacean/embedded-rust-embassy-pwm-generation-15cf <- better?
#[embassy_executor::task]
pub async fn piezo (
    pm: PWM0,
    pin: AnyPin
) {
    info!("{}: start", TASK_ID);

    // note, frequency (hz)
    let tones = [
        ('c', 261),
        ('d', 294),
        ('e', 329),
        ('f', 349),
        ('g', 392),
        ('a', 440),
        ('b', 493),
    ];

    // note, duration
    let tune = [
        ('c', 1),
        ('c', 1),
        ('g', 1),
        ('g', 1),
        ('a', 1),
        ('a', 1),
        ('g', 2),
        ('f', 1),
        ('f', 1),
        ('e', 1),
        ('e', 1),
        ('d', 1),
        ('d', 1),
        ('c', 2),
        (' ', 4),
    ];

    let tempo = 300_u64; // ms


    let mut pwm = SimplePwm::new_1ch(pm, pin);
    pwm.set_ch0_drive(OutputDrive::Standard);
    pwm.set_prescaler(Prescaler::Div1);
    pwm.set_max_duty(32767);
    pwm.set_duty(0, 32767 / 2); // 50% of max duty, this should be volume?
    pwm.enable();
return;

    loop {
        info!("loop start");

        // 1. Obtain a note in the tune
        for note in tune {
            info!("note: {}", note.0);
            
            // 2. Retrieve the freqeuncy and beat associated with the note
            for tone in tones {
                // 2.1 Find a note match in the tones array and update frequency and beat variables accordingly
                if tone.0 == note.0 {
                    info!("tone: {}", tone.0);
                    // 3. Play the note for the desired duration (beats*tempo)
                    // 3.1 Adjust period of the PWM output to match the new frequency
                    pwm.set_period(tone.1);
 
                    // 3.2 Enable the channel to generate desired PWM
                    pwm.enable();

                    // 3.3 Keep the output on for as long as required
                    Timer::after_millis(note.1 * tempo).await;
                } else if note.0 == ' ' {
                    // 2.2 if ' ' tone is found disable output for one beat
                    pwm.disable();
                    Timer::after_millis(tempo).await;
                }
            }
            // 4. Silence for half a beat between notes
            // 4.1 Disable the PWM output (silence)
            pwm.disable();
            // 4.2 Keep the output off for half a beat between notes
            Timer::after_millis(tempo / 2).await;
            // 5. Go back to 1.
        }
    }
}