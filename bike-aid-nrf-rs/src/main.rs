/*

Pin Guide

P0.06 - Throttle
P1.11 - LED
P1.15 - SPEED
P0.29 - TWI SDA
P0.31 - TWI SCL

*/

#![no_std]
#![no_main]

// modules/creates
mod signals;

mod device_dac;

mod task_clock;
mod task_led;
mod task_temperature;
mod task_speed;
mod task_battery;
mod task_alarm;
mod task_throttle;
mod task_bluetooth;

// external imports
use embassy_nrf::{gpio::Pin, temp::Temp};
use embassy_time::Timer;
use embassy_executor::Spawner;
use defmt::*;
use {defmt_rtt as _, panic_probe as _};


#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());

    // DEBUG: add sleep incase we need to flash during debug and get a crash
    Timer::after_secs(2).await;

    // Configure and setup shared async I2C/TWI communication
    let mut shared_twi = {
        use embassy_nrf::{bind_interrupts, peripherals::{self}, twim::{self, Twim}};
        bind_interrupts!(struct Irqs {
            SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0 => twim::InterruptHandler<peripherals::TWISPI0>;
        });
        let port_twi = p.TWISPI0;
        let pin_sda = p.P0_29.degrade();
        let pin_scl = p.P0_31.degrade();
        let config = twim::Config::default();
        Twim::new(port_twi, Irqs, pin_sda, pin_scl, config)
        //SHARED_ASYNC_I2C.init(Mutex::new(i2c))
    };

    // scan for i2c/twi devices
    for address in 1..128 {
        match shared_twi.write(address, &[]).await {
            Ok(_) => {
                info!("Device found at address: 0x{:X}", address);
            }
            Err(_) => continue,
        }
    }

    // INIT DEVICES
    use crate::device_dac::dac;
    spawner.must_spawn(dac(
        shared_twi,
        0x60,
    ));
    

    // INIT TASKS

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
    let t = {
        use embassy_nrf::{bind_interrupts, temp};
        bind_interrupts!(struct Irqs {
            TEMP => temp::InterruptHandler;
        });
        Temp::new(p.TEMP, Irqs)
    };
    use crate::task_temperature::temperature;
    spawner.must_spawn(temperature(
        t
    ));

    // Alarm Task
    use crate::task_alarm::alarm;
    spawner.must_spawn(alarm());

    // Bluetooth Task
    use crate::task_bluetooth::bluetooth;
    spawner.must_spawn(bluetooth());
 
    // Throttle Task
    let saadc = {
        use embassy_nrf::saadc::{ChannelConfig, Config, Saadc};
        use embassy_nrf::{bind_interrupts, saadc};
        bind_interrupts!(struct Irqs {
            SAADC => saadc::InterruptHandler;
        });
        let config = Config::default();
        let mut pin = p.P0_02;
        let channel_config = ChannelConfig::single_ended(&mut pin);
        Saadc::new(p.SAADC, Irqs, config, [channel_config])   
    };
    use crate::task_throttle::throttle;
    spawner.must_spawn(throttle(
        saadc
    ));

    // loop for testing
    let pub_led = signals::LED_MODE.publisher().unwrap();
    let mut sub_minutes = signals::CLOCK_MINUTES.subscriber().unwrap();
    loop {
        let val = sub_minutes.next_message_pure().await;
        pub_led.publish_immediate(task_led::LedMode::OnOffSlow);
        info!("{:02}", val);
    }
 
}