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


HW Todo
----------
try a smaller pulldown on the throttle module, replace 100k with 47k to see if it helps with floating throttle
brake plug seems wobbly
reset push button - hw
speedo - hardware/oscilliscope
brake supply 5v with diode to drop 0.7v. then can setup parkbrake to turn off power.


App Todo
----------
park brake status
brake status
cruise status


Todo
----------
auto off after x mins of park brake?
try use a watch instead of both pubsub & mutex for power, alarm, settings(settings change, restart throttle??) etc..

alarm
power meter
cruise 1,2 restore speed if brake is less than 3 seconds?
double tap cruise current speed. store initial voltage at the start of the tap detection
custom voltage from the app could override the cruise? need an extra mutex for that
command que for ble
ble tracker
odometer/speed

*/

#![no_std]
#![no_main]

// modules/creates
mod tasks;
mod examples;
mod ble;
mod utils;
use crate::tasks::*;
use crate::utils::signals;

// external imports
use core::cell::RefCell;
use cortex_m::asm::nop;
use defmt::info;
use embassy_nrf::interrupt::{self, InterruptExt};
use embassy_nrf::{bind_interrupts, config::Reg0Voltage, gpio::Pin, interrupt::Priority, peripherals::TWISPI0};
use embassy_nrf::peripherals::{self};
use embassy_nrf::nvmc::Nvmc;
use embassy_time::Timer;
use embassy_executor::Spawner;
use defmt_rtt as _;
//use panic_probe as _; // this should reset device on panic!
use core::panic::PanicInfo;


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

    // init default signals
    signals::init();

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

    spawner.must_spawn(park_brake::task());

    spawner.must_spawn(speed::task(p.P0_09.degrade()));

    spawner.must_spawn(switch_power::task(p.P0_10.degrade()));

    spawner.must_spawn(power_down::task());

    spawner.must_spawn(manual_override::task(p.P0_20.degrade()));

    //spawner.must_spawn(switch_horn::task(p.P1_11.degrade()));

    //spawner.must_spawn(switch_light::task(p.P0_10.degrade()));

    spawner.must_spawn(battery::task());

    // disable when debug
    spawner.must_spawn(piezo::task(p.PWM0, p.P0_29.degrade()));

    spawner.must_spawn(alarm::task());

    spawner.must_spawn(throttle::task());

    spawner.must_spawn(cruise::task());

    spawner.must_spawn(bluetooth::task(spawner));

    spawner.must_spawn(cli::task());

    spawner.must_spawn(led::task(p.P0_31.degrade())); // 0.31, 0.15

    spawner.must_spawn(clock::task());

    Timer::after_millis(100).await;
    info!("======== Boot Ok ========");

    // boot ok feedback
    // single blink led
    // boot tune
    let pub_led = signals::LED_MODE.publisher().unwrap();
    let pub_piezo = signals::PIEZO_MODE.publisher().unwrap();
    pub_led.publish_immediate(signals::LedModeType::SingleSlow);
    pub_piezo.publish_immediate(signals::PiezoModeType::Boot);



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
    
    // 
    // let mut sub_minutes = signals::CLOCK_MINUTES.subscriber().unwrap();
    // loop {
    //     let val = sub_minutes.next_message_pure().await;
    //     info!("Main - Time: {}", val);
    // }
}


#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    cortex_m::interrupt::disable();
    defmt::error!("Panicked: {}", defmt::Display2Format(info));
    //panic_persist::report_panic_info(info);
    for _ in 0..2_000_000 { // delay before reset
        nop()
    }
    cortex_m::peripheral::SCB::sys_reset();
}