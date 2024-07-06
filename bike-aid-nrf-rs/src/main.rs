/*

Pin Guide

P1.11 - LED
P1.15 - SPEED
P0.06 - I2C/TWI SDA
P0.08 - I2C/TWI SCL

*/

#![no_std]
#![no_main]

// modules/creates
mod signals;
mod functions;
mod store;

mod device_throttle_dac;
mod device_throttle_adc;

mod task_store;
mod task_clock;
mod task_led;
mod task_temperature;
mod task_speed;
mod task_battery;
mod task_alarm;
mod task_throttle;
mod task_bluetooth;

use core::cell::RefCell;

// external imports
use embassy_nrf::{gpio::Pin, interrupt::{self, Priority}, peripherals::TWISPI0};
use embassy_nrf::nvmc::Nvmc;
use embassy_time::Timer;
use embassy_executor::Spawner;
use defmt::*;
use {defmt_rtt as _, panic_probe as _};
//use rtt_target::{rprintln, rtt_init_print};

// Static i2c/twi mutex for shared-bus functionality
use static_cell::StaticCell;
use embassy_nrf::twim::{self, Twim};
use embassy_embedded_hal::shared_bus::blocking::i2c::I2cDevice;
use embassy_sync::blocking_mutex::NoopMutex;
use embedded_hal::i2c::I2c;

// blocking mutex for shared-bus
static I2C_BUS: StaticCell<NoopMutex<RefCell<Twim<TWISPI0>>>> = StaticCell::new();


#[embassy_executor::main]
async fn main(spawner: Spawner) {
    // configure for softdevice
    // interrupt levels 0, 1 and 4 are reserved by the softdevice
    let mut config = embassy_nrf::config::Config::default();
    config.gpiote_interrupt_priority = Priority::P2;
    config.time_interrupt_priority = Priority::P2;
    let p = embassy_nrf::init(config);


    //let p = embassy_nrf::init(Default::default());

    // DEBUG: add sleep incase we need to flash during debug and get a crash
    Timer::after_secs(2).await;

  
    // shared i2c/twi bus
    let i2c_bus = {
        use embassy_nrf::{bind_interrupts, peripherals::{self}};
        //use embassy_nrf::interrupt::Interrupt;
        //embassy_nrf::interrupt::SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0::set_priority(interrupt::Priority::P2);
        bind_interrupts!(struct Irqs {SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0 => twim::InterruptHandler<peripherals::TWISPI0>;});
        let config = twim::Config::default();
        let i2c = Twim::new(p.TWISPI0, Irqs, p.P0_06, p.P0_08, config); // sda: p0.06, scl: p0.08
        let i2c_bus = NoopMutex::new(RefCell::new(i2c));
        I2C_BUS.init(i2c_bus)
    };
  

    /*
    // Debug: scan for i2c/twi devices
    // this crashes with 5v on the dac for some reason?
    // doesnt work with the logic converter either!
    let mut i2c_dev1 = I2cDevice::new(i2c_bus);
    for address in 1..128 {
        let result = i2c_dev1.write(address, &[]);
        match result {
            Ok(_) => {info!("I2C/TWI found device: 0x{:X}", address);}
            Err(_) => continue,
        }
    }
     */
    /*

    // INIT DEVICES

    // Throttle ADC (input)
    use crate::device_throttle_adc::adc;
    spawner.must_spawn(adc(
        I2cDevice::new(i2c_bus)
    ));
 */
    /*
    // Throttle ADC (output)
    use crate::device_throttle_dac::dac;
    spawner.must_spawn(dac(
        I2cDevice::new(i2c_bus)
    ));
 */

    // INIT TASKS

    // Store Task
    use crate::task_store::store;
    spawner.must_spawn(store(
        Nvmc::new(p.NVMC)
    ));

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
    spawner.must_spawn(temperature(
        p.TEMP
    ));

    // Alarm Task
    use crate::task_alarm::alarm;
    spawner.must_spawn(alarm());

    // Throttle Task
    use crate::task_throttle::throttle;
    spawner.must_spawn(throttle());

    /*
    // Bluetooth Task
    use crate::task_bluetooth::bluetooth;
    spawner.must_spawn(bluetooth(
        spawner
    ));
     */
    
     

    // loop for testing
    let pub_led = signals::LED_MODE.publisher().unwrap();
    let mut sub_minutes = signals::CLOCK_MINUTES.subscriber().unwrap();
    loop {
        let val = sub_minutes.next_message_pure().await;
        pub_led.publish_immediate(task_led::LedMode::OnOffSlow);
        info!("Clock: {:02}", val);
    }
 
}