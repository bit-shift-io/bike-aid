/*

Pin Map
=====================
p0.03 - twi/i2c SDA
p0.04 - twi/i2c SCL
p1.11 - led
p1.15 - speed

*/


#![no_std]
#![no_main]
//#[panic_handler] fn panic(_info: &core::panic::PanicInfo) -> ! { loop {} }

// modules/creates
mod signals;
mod task_clock;
mod task_twm;
mod task_led;
mod task_temperature;
mod task_speed;
mod task_battery;
mod task_alarm;
mod task_throttle;


// external imports
use embassy_executor::Spawner;
use defmt::*;
use embassy_nrf::gpio::Pin;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());

    // Clock Task
    use crate::task_clock::clock;
    spawner.must_spawn(clock());

    // LED Task
    use crate::task_led::led;
    spawner.must_spawn(led(
        p.P1_11.degrade() // label 111 - D14
    ));

    // Temperature Task
    use crate::task_temperature::temperature;
    spawner.must_spawn(temperature(
        p.P0_03.degrade()
    ));  

    // Battery Task
    use crate::task_battery::battery;
    spawner.must_spawn(battery()); 

    // Throttle Task
    use crate::task_throttle::throttle;
    spawner.must_spawn(throttle());

    // Alarm Task
    use crate::task_alarm::alarm;
    spawner.must_spawn(alarm());

    // Speed Task
    use crate::task_speed::speed;
    spawner.must_spawn(speed(
        p.P1_15.degrade() // label 115 - D18
    ));

    // TWM Task
    use crate::task_twm::twm;
    spawner.must_spawn(twm());



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