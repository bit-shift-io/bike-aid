/*
Pin Guide
----------
P0.31 - LED
P0.29 - Piezo
P0.20 - Manual Override
P0.17 - Brake
P0.09 - Speed
P0.10 - Power Switch

//P0.10 - Light
//P1.11 - Horn

P0.06 - I2C SCL - Orange ( Green on breadboard )
P0.08 - I2C SDA - Yellow

Notes
----------
nfc-pins-as-gpio Allow using the NFC pins as regular GPIO P0_09/P0_10 on nRF52
reset-pin-as-gpio Allow using the RST pin as a regular GPIO P0_18

P0.13 controls vcc output on/off 3.3v
P0.14-0.16 set low resets ?
p0.15 Debug LED

Todo
----------
fix ble power on/off (phone issue?)
battery power reading
odometer
alarm on/off/beeps
cruise - auto

alarm - auto on/off
speedo
*/

#![no_std]
#![no_main]

// modules/creates
mod tasks;
mod examples;
mod ble;
mod utils;
use crate::tasks::*;

// external imports
use core::cell::RefCell;
use embassy_nrf::interrupt::{self, InterruptExt};
use embassy_nrf::{bind_interrupts, config::Reg0Voltage, gpio::Pin, interrupt::Priority, peripherals::TWISPI0};
use embassy_nrf::peripherals::{self};
use embassy_nrf::nvmc::Nvmc;
use embassy_time::Timer;
use embassy_executor::Spawner;
use defmt::*;
use {defmt_rtt as _, panic_probe as _};

// Static i2c/twi mutex for shared-bus functionality
use static_cell::StaticCell;
use embassy_nrf::twim::{self, Twim};
use embassy_sync::blocking_mutex::NoopMutex;

// blocking mutex for shared-bus
static I2C_BUS: StaticCell<NoopMutex<RefCell<Twim<TWISPI0>>>> = StaticCell::new();

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("======== Starting ========");
    
    let config = {
        let mut c = embassy_nrf::config::Config::default();
        // change interrupts for softdevice - levels 0, 1 and 4 are reserved by the softdevice
        c.gpiote_interrupt_priority = Priority::P2;
        c.time_interrupt_priority = Priority::P2;
        // voltage from 2.8v to 3.3v
        c.dcdc.reg0 = true;
        c.dcdc.reg0_voltage = Some(Reg0Voltage::_3v3);
        c
    };

    let p = embassy_nrf::init(config);

    // add sleep incase we need to flash during debug and get a crash
    Timer::after_secs(2).await;

    // shared i2c/twi bus
    let i2c_bus = {
        bind_interrupts!(struct Irqs {SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0 => twim::InterruptHandler<peripherals::TWISPI0>;});
        interrupt::SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0.set_priority(interrupt::Priority::P3);
        let config = twim::Config::default();
        let i2c = Twim::new(p.TWISPI0, Irqs, p.P0_08, p.P0_06, config); // sda: p0.08, scl: p0.06
        let i2c_bus = NoopMutex::new(RefCell::new(i2c));
        I2C_BUS.init(i2c_bus)
    };

    // == INIT DEVICES ==
    // Causes issues if there is no pullups for the i2c bus

    spawner.must_spawn(throttle_adc::task(i2c_bus));

    spawner.must_spawn(throttle_dac::task(i2c_bus));

    spawner.must_spawn(gyroscope::task(i2c_bus));

    spawner.must_spawn(temperature::task(i2c_bus));

    spawner.must_spawn(battery_adc::task(i2c_bus));

    // == INIT TASKS ==

    spawner.must_spawn(store::task(Nvmc::new(p.NVMC)));

    spawner.must_spawn(brake::task(p.P0_17.degrade()));

    spawner.must_spawn(speed::task(p.P0_09.degrade()));

    spawner.must_spawn(switch_power::task(p.P0_10.degrade()));

    spawner.must_spawn(manual_override::task(p.P0_20.degrade()));

    //spawner.must_spawn(switch_horn::task(p.P1_11.degrade()));

    //spawner.must_spawn(switch_light::task(p.P0_10.degrade()));

    spawner.must_spawn(battery::task());

    spawner.must_spawn(piezo::task(p.PWM0, p.P0_29.degrade()));

    spawner.must_spawn(alarm::task(spawner));

    spawner.must_spawn(throttle::task());

    spawner.must_spawn(bluetooth::task(spawner));

    spawner.must_spawn(cli::task());

    spawner.must_spawn(led::task(p.P0_31.degrade())); // 0.31, 0.15

    spawner.must_spawn(clock::task());

    Timer::after_millis(100).await;
    info!("======== Boot Ok ========");


    // == DEBUG ==

    //use embassy_nrf::gpio::{Level, Output, OutputDrive};
    //Output::new(p.P0_14, Level::Low, OutputDrive::Standard);
    //Output::new(p.P0_15, Level::Low, OutputDrive::Standard);
    //Output::new(p.P0_16, Level::Low, OutputDrive::Standard);

    // spawner.must_spawn(fake_signals::task());

    //use crate::examples::blinky;
    //spawner.must_spawn(blinky::task(p.P0_03.degrade()));

    // use crate::examples::i2c_scan;
    // spawner.must_spawn(i2c_scan::task(i2c_bus));

    // turn device on for testing
    //Timer::after_millis(100).await;
    //let pub_power = signals::SWITCH_POWER.publisher().unwrap();
    //pub_power.publish(true).await;
    
    use crate::utils::signals;
    let mut sub_minutes = signals::CLOCK_MINUTES.subscriber().unwrap();
    loop {
        let val = sub_minutes.next_message_pure().await;
        info!("Main - Time: {}", val);
    }
}