/*

Pin Guide

P1.11 - LED
P1.15 - SPEED
P0.03 - TWI SDA
P0.04 - TWI SCL

*/

#![no_std]
#![no_main]

// modules/creates
mod signals;
mod task_clock;
mod task_twi_manager;
mod task_led;
mod task_temperature;
mod task_speed;
mod task_battery;
mod task_alarm;
mod task_throttle;
mod task_bluetooth;

// external imports
use embassy_nrf::gpio::Pin;
use embassy_time::Timer;
use embassy_executor::Spawner;
use defmt::*;
use {defmt_rtt as _, panic_probe as _};


#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());

    // DEBUG: add sleep incase we need to flash during debug and get a crash
    Timer::after_secs(2).await;

    // Clock Task
    use crate::task_clock::clock;
    spawner.must_spawn(clock());

    // LED Task
    use crate::task_led::led;
    spawner.must_spawn(led(
        p.P1_11.degrade() // label 111 - D14
    ));

    // Speed Task
    use crate::task_speed::speed;
    spawner.must_spawn(speed(
        p.P1_15.degrade() // label 115
    ));

    // Battery Task
    use crate::task_battery::battery;
    spawner.must_spawn(battery());

    // Temperature Task
    use crate::task_temperature::temperature;
    spawner.must_spawn(temperature());

    // Alarm Task
    use crate::task_alarm::alarm;
    spawner.must_spawn(alarm());

    // Throttle Task
    use crate::task_throttle::throttle;
    spawner.must_spawn(throttle());

    // Bluetooth Task
    use crate::task_bluetooth::bluetooth;
    spawner.must_spawn(bluetooth());
 
 /*
    // TWI
    use crate::task_twi_manager::twi_manager;
    spawner.must_spawn(twi_manager(
        p.TWISPI0,
        p.P0_03.degrade(),
        p.P0_04.degrade()
    ));

 */
    // test loop
    // loop
    let mut sub_minutes = signals::CLOCK_MINUTES.subscriber().unwrap();
    let mut pub_led = signals::LED_MODE.publisher().unwrap();
    loop {
        let val = sub_minutes.next_message_pure().await;
        pub_led.publish_immediate(task_led::LedMode::ThreeFast);
        info!("{:02}", val);
    }
 
}