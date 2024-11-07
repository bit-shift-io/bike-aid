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
disconnect while connecting causes multiple instances of scanner

Todo
----------
attach debugger to running mcu
try use a watch  for settings(settings change, restart throttle??) etc..
alarm
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
use embassy_sync::mutex;
use tasks::*;
use utils::signals;

use defmt::info;
use embassy_nrf::config::Config;
use embassy_nrf::gpio::AnyPin;
use embassy_nrf::{bind_interrupts, config::Reg0Voltage, gpio::Pin};
use embassy_nrf::peripherals::{self, TWISPI0};
use embassy_nrf::nvmc::Nvmc;
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_time::Timer;
use embassy_nrf::interrupt;
use embassy_nrf::interrupt::{InterruptExt, Priority};
use embassy_executor::{InterruptExecutor, SendSpawner, Spawner};
use embassy_nrf::twim::{self, Twim};
use static_cell::StaticCell;


#[embassy_executor::main]
async fn main(spawner: Spawner) {
    rtt::init(spawner);
    info!("======== Starting ========");
    let p = embassy_nrf::init(get_config()); // make mut if need be for shared resources
    Timer::after_secs(2).await; // sleep incase we need to flash during debug and get a crash
    signals::init();
    let (spawn_high, spawn_med) = init_priority_spawners();
   

    // == I2C DEVICES ==

    let i2c_bus = init_async_i2c(p.TWISPI0, p.P0_08.degrade(), p.P0_06.degrade());

    spawner.must_spawn(throttle_adc::task(i2c_bus));
    spawner.must_spawn(throttle_dac::task(i2c_bus));
    spawner.must_spawn(gyroscope::task(i2c_bus));
    spawner.must_spawn(temperature::task(i2c_bus));
    spawner.must_spawn(battery_adc::task(i2c_bus));
 
    // == TASKS ==

    spawner.must_spawn(switch_power::task(p.P0_10.degrade()));
    spawner.must_spawn(cli::task());
    spawner.must_spawn(brake::task(p.P0_17.degrade()));
    spawner.must_spawn(park_brake::task());
    spawner.must_spawn(store::task(Nvmc::new(p.NVMC)));
    spawner.must_spawn(speed::task(p.P0_09.degrade()));
    spawner.must_spawn(power_down::task());
    spawner.must_spawn(manual_override::task(p.P0_20.degrade()));
    spawner.must_spawn(battery::task());
    spawner.must_spawn(piezo::task(p.PWM0, p.P0_29.degrade())); // disable when debug to mute
    spawner.must_spawn(alarm::task());
    spawner.must_spawn(throttle::task());
    spawner.must_spawn(cruise::task());
    spawner.must_spawn(bluetooth::task(spawner));
    spawner.must_spawn(led::task(p.P0_31.degrade(), 0));
    spawner.must_spawn(led::task(p.P0_15.degrade(), 1));
    spawner.must_spawn(clock::task());
    //spawner.must_spawn(switch_horn::task(p.P1_11.degrade()));
    //spawner.must_spawn(switch_light::task(p.P0_10.degrade()));

    // == FINALISE ==

    boot_ok().await;
    debug().await;
}


fn init_priority_spawners() -> (SendSpawner, SendSpawner) {
    static EXECUTOR_HIGH: InterruptExecutor = InterruptExecutor::new();
    static EXECUTOR_MED: InterruptExecutor = InterruptExecutor::new();

    #[interrupt]
    unsafe fn SWI1_EGU1() {
        EXECUTOR_HIGH.on_interrupt()
    }

    #[interrupt]
    unsafe fn SWI0_EGU0() {
        EXECUTOR_MED.on_interrupt()
    }

    // High-priority executor: SWI1_EGU1, priority level 6
    interrupt::SWI1_EGU1.set_priority(Priority::P6);
    let spawner_high = EXECUTOR_HIGH.start(interrupt::SWI1_EGU1);

    // Medium-priority executor: SWI0_EGU0, priority level 7
    interrupt::SWI0_EGU0.set_priority(Priority::P7);
    let spawner_med = EXECUTOR_MED.start(interrupt::SWI0_EGU0);

    (spawner_high, spawner_med)
}


fn get_config() -> Config {
    let mut c = Config::default();
    // change interrupts for softdevice - levels 0, 1 and 4 are reserved by the softdevice
    c.gpiote_interrupt_priority = Priority::P2;
    c.time_interrupt_priority = Priority::P2;
    // voltage from 2.8v to 3.3v
    c.dcdc.reg0 = true;
    c.dcdc.reg0_voltage = Some(Reg0Voltage::_3v3);
    c
}


fn init_async_i2c(twim: TWISPI0, sda: AnyPin, scl: AnyPin) -> &'static mut mutex::Mutex<ThreadModeRawMutex, Twim<'static, TWISPI0>> {
    bind_interrupts!(struct Irqs {SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0 => twim::InterruptHandler<peripherals::TWISPI0>;});
    interrupt::SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0.set_priority(interrupt::Priority::P3);
    
    let config = twim::Config::default();
    let i2c = Twim::new(twim, Irqs, sda, scl, config);
    let i2c_bus = mutex::Mutex::<ThreadModeRawMutex, _>::new(i2c);
    // note, we can place a refcell around the twim bus to allow it to be shared between tasks
    static ASYNC_I2C_BUS: StaticCell<mutex::Mutex<ThreadModeRawMutex, Twim<TWISPI0>>> = StaticCell::new();
    let result: &mut mutex::Mutex<ThreadModeRawMutex, Twim<'_, TWISPI0>> = ASYNC_I2C_BUS.init(i2c_bus);
    result
}


// fn init_i2c_bus(twim: TWISPI0, sda: AnyPin, scl: AnyPin) -> &'static mut Mutex<NoopRawMutex, RefCell<Twim<'static, TWISPI0>>> {
//     bind_interrupts!(struct Irqs {SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0 => twim::InterruptHandler<peripherals::TWISPI0>;});
//     interrupt::SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0.set_priority(interrupt::Priority::P3);
    
//     let config = twim::Config::default();
//     let i2c = Twim::new(twim, Irqs, sda, scl, config); // sda: p0.08, scl: p0.06
//     let i2c_bus = NoopMutex::new(RefCell::new(i2c));
    
//     static I2C_BUS: StaticCell<NoopMutex<RefCell<Twim<TWISPI0>>>> = StaticCell::new();
//     I2C_BUS.init(i2c_bus)
// }


async fn boot_ok() {
    Timer::after_millis(100).await;
    info!("======== Boot Ok ========");
    let send_led = signals::LED_MODE.sender();
    let send_piezo = signals::PIEZO_MODE.sender();
    send_led.send(signals::LedModeType::SingleSlow);
    send_piezo.send(signals::PiezoModeType::Boot);
}


async fn debug() {
    //Timer::after_millis(100).await;
    //info!("======== Debug ========");

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
    //let send_power = signals::POWER_ON.sender();
    //send_power.send(true);
}