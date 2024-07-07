/*

Pin Guide

P1.11 - LED
P1.06 - Button
P1.15 - Speed
P0.06 - I2C/TWI SDA
P0.08 - I2C/TWI SCL
P1.04 - Relay Power

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
mod task_button;
mod task_relay_power;

use core::cell::RefCell;

// external imports
use embassy_nrf::{bind_interrupts, config::{DcdcConfig, Reg0Voltage}, gpio::{Level, Output, OutputDrive, Pin}, interrupt::{self, Priority}, peripherals::TWISPI0, saadc::{self, ChannelConfig, Config, Saadc}};
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

    let mut config = embassy_nrf::config::Config::default();

    // change interrupts for softdevice
    // interrupt levels 0, 1 and 4 are reserved by the softdevice
    config.gpiote_interrupt_priority = Priority::P2;
    config.time_interrupt_priority = Priority::P2;

    // change default pin voltage from 2.8v to 3.3v
    config.dcdc.reg0 = true;
    config.dcdc.reg0_voltage = Some(Reg0Voltage::_3v3);

    let p = embassy_nrf::init(config);

    // add sleep incase we need to flash during debug and get a crash
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
        p.P1_11.degrade()
    ));

    // Button Task
    use crate::task_button::button;
    spawner.must_spawn(button(
        p.P1_06.degrade()
    ));

    // Relay Power Task
    use crate::task_relay_power::relay_power;
    spawner.must_spawn(relay_power(
        p.P0_04.degrade()
    ));

    // Speed Task
    use crate::task_speed::speed;
    spawner.must_spawn(speed(
        p.P1_15.degrade()
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
    

    
    // TODO: test pin 13 controls vcc output on/off 3.3v apparently?
    //Output::new(p.P0_13, Level::Low, OutputDrive::Standard);


/*
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

    // test saadc with fixed voltage
    bind_interrupts!(struct Irqs {
        SAADC => saadc::InterruptHandler;
    });
    let config = Config::default();
    let channel_config = ChannelConfig::single_ended(p.P0_02);
    let mut saadc = Saadc::new(p.SAADC, Irqs, config, [channel_config]);
    saadc.calibrate().await;
    Timer::after_millis(500).await;

    loop {
        let mut buf = [0; 1];
        saadc.sample(&mut buf).await;
        let input = buf[0];
        let voltage = f32::from(input) * 3600.0 / 4096.0; // converted to mv
        info!("sample: {}", voltage);
        Timer::after_millis(100).await;
    }
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