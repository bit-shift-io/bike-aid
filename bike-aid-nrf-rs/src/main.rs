/*

Pin Guide

P1.11 - LED
P1.06 - Brake
P1.15 - Speed
P0.09 - Piezo

P0.06 - I2C/TWI SDA
P0.08 - I2C/TWI SCL

P1.11 - Power Switch
P1.04 - Light
P1.00 - Horn

nfc-pins-as-gpio Allow using the NFC pins as regular GPIO pins (P0_09/P0_10 on nRF52, P0_02/P0_03 on nRF53)
reset-pin-as-gpio Allow using the RST pin as a regular GPIO pin.
 * nRF52805, nRF52810, nRF52811, nRF52832: P0_21
 * nRF52820, nRF52833, nRF52840: P0_18

// TODO: test pin 13 controls vcc output on/off 3.3v apparently?
//Output::new(p.P0_13, Level::Low, OutputDrive::Standard);
*/

#![no_std]
#![no_main]

// modules/creates
mod tasks;
mod examples;
mod ble;
mod utils;

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
use embassy_embedded_hal::shared_bus::blocking::i2c::I2cDevice;
use embassy_sync::blocking_mutex::NoopMutex;

// blocking mutex for shared-bus
static I2C_BUS: StaticCell<NoopMutex<RefCell<Twim<TWISPI0>>>> = StaticCell::new();

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("======== Starting ========");
    
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
        bind_interrupts!(struct Irqs {SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0 => twim::InterruptHandler<peripherals::TWISPI0>;});
        interrupt::SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0.set_priority(interrupt::Priority::P3);
        let config = twim::Config::default();
        let i2c = Twim::new(p.TWISPI0, Irqs, p.P0_06, p.P0_08, config); // sda: p0.06, scl: p0.08
        let i2c_bus = NoopMutex::new(RefCell::new(i2c));
        I2C_BUS.init(i2c_bus)
    };


    // == DEBUG ==

    // send signals
    use crate::examples::fake_signals::fake_signals;
    spawner.must_spawn(fake_signals());
    
    // scan i2c devices
    use crate::examples::i2c_scan::scan;
    spawner.must_spawn(scan(
        I2cDevice::new(i2c_bus)
    ));


    // == INIT DEVICES ==

    // Throttle ADC (input)
    use crate::tasks::throttle_adc::throttle_adc;
    spawner.must_spawn(throttle_adc(
        I2cDevice::new(i2c_bus)
    ));

    // Throttle ADC (output)
    use crate::tasks::throttle_dac::throttle_dac;
    spawner.must_spawn(throttle_dac(
        I2cDevice::new(i2c_bus)
    ));

    // Gyroscope + Temperature
    use crate::tasks::gyroscope::gyroscope;
    spawner.must_spawn(gyroscope(
        I2cDevice::new(i2c_bus)
    ));

    // Battery ADC Task
    use crate::tasks::battery_adc::battery_adc;
    spawner.must_spawn(battery_adc(
        I2cDevice::new(i2c_bus)
    ));
    

    // == INIT TASKS ==

    // Store Task
    use crate::tasks::store::store;
    spawner.must_spawn(store(
        Nvmc::new(p.NVMC)
    ));

    // Clock Task
    use crate::tasks::clock::clock;
    spawner.must_spawn(clock());

    // LED Task
    use crate::tasks::led::led;
    spawner.must_spawn(led(
        p.P1_11.degrade()
    ));

    // Brake Task
    use crate::tasks::brake::brake;
    spawner.must_spawn(brake(
        p.P1_06.degrade()
    ));

    // Power Switch Task
    use crate::tasks::switch_power::switch_power;
    spawner.must_spawn(switch_power(
        p.P0_11.degrade()
    ));

    // Horn Switch Task
    use crate::tasks::switch_horn::switch_horn;
    spawner.must_spawn(switch_horn(
        p.P1_04.degrade()
    ));

    // Light Switch Task
    use crate::tasks::switch_light::switch_light;
    spawner.must_spawn(switch_light(
        p.P1_00.degrade()
    ));

    // Speed Task
    use crate::tasks::speed::speed;
    spawner.must_spawn(speed(
        p.P1_15.degrade()
    ));

    // Battery Task
    use crate::tasks::battery::battery;
    spawner.must_spawn(battery());

    // Piezo Task
    use crate::tasks::piezo::piezo;
    spawner.must_spawn(piezo(
        p.PWM0,
        p.P0_09.degrade()
    ));

    // Alarm Task
    use crate::tasks::alarm::alarm;
    spawner.must_spawn(alarm(
        spawner
    ));

    // Throttle Task
    use crate::tasks::throttle::throttle;
    spawner.must_spawn(throttle());

    // Bluetooth Task
    use crate::tasks::bluetooth::bluetooth;
    spawner.must_spawn(bluetooth(
        spawner
    ));

    // CLI Task
    use crate::tasks::cli::cli;
    spawner.must_spawn(cli());

    // == TEST ==

    // loop for testing
    use utils::signals;
    let mut sub_minutes = signals::CLOCK_MINUTES.subscriber().unwrap();
    loop {
        let val = sub_minutes.next_message_pure().await;
        info!("Clock: {:02}", val);
    }
}