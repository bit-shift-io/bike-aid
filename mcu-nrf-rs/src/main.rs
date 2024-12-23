#![no_std]
#![no_main]

// modules/creates
mod tasks;
mod examples;
mod ble;
mod utils;

use tasks::*;
use utils::{i2c, signals};

use defmt::info;
use embassy_nrf::config::Config;
use embassy_nrf::{config::Reg0Voltage, gpio::Pin};
use embassy_time::Timer;
use embassy_nrf::interrupt::Priority;
use embassy_executor::Spawner;


#[embassy_executor::main]
async fn main(spawner: Spawner) {

    // == INIT ==

    rtt::init(spawner);
    info!("======== Starting ========");
    let p = embassy_nrf::init(get_config()); // make mut if need be for shared resources
    Timer::after_secs(2).await; // sleep incase we need to flash during debug and get a crash
    //let (spawn_high, spawn_med) = init_priority_spawners();
   
    // == I2C DEVICES ==

    let i2c_bus = i2c::init(p.TWISPI0, p.P0_08.degrade(), p.P0_06.degrade());

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

    // == FINALISE ==

    boot_ok().await;
    debug(spawner).await;
}


// #[allow(dead_code)]
// fn init_priority_spawners() -> (SendSpawner, SendSpawner) {
//     static EXECUTOR_HIGH: InterruptExecutor = InterruptExecutor::new();
//     static EXECUTOR_MED: InterruptExecutor = InterruptExecutor::new();

//     #[interrupt]
//     unsafe fn SWI1_EGU1() {
//         EXECUTOR_HIGH.on_interrupt()
//     }

//     #[interrupt]
//     unsafe fn SWI0_EGU0() {
//         EXECUTOR_MED.on_interrupt()
//     }

//     // High-priority executor: SWI1_EGU1, priority level 6
//     interrupt::SWI1_EGU1.set_priority(Priority::P6);
//     let spawner_high = EXECUTOR_HIGH.start(interrupt::SWI1_EGU1);

//     // Medium-priority executor: SWI0_EGU0, priority level 7
//     interrupt::SWI0_EGU0.set_priority(Priority::P7);
//     let spawner_med = EXECUTOR_MED.start(interrupt::SWI0_EGU0);

//     (spawner_high, spawner_med)
// }


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


async fn boot_ok() {
    Timer::after_millis(10).await;
    info!("======== Boot Ok ========");
    let send_led = signals::LED_MODE.sender();
    let send_piezo = signals::PIEZO_MODE.sender();
    send_led.send(signals::LedModeType::SingleSlow);
    send_piezo.send(signals::PiezoModeType::Boot);
}


async fn debug(_spawner: Spawner) {
    // Timer::after_millis(10).await;
    // info!("======== Debug ========");

    // use examples::signal_test;
    // spawner.must_spawn(signal_test::task(spawner));
    
    // use crate::examples::i2c_scan;
    // spawner.must_spawn(i2c_scan::task(i2c_bus));

    // turn device on for testing
    //Timer::after_millis(100).await;
    //let send_power = signals::POWER_ON.sender();
    //send_power.send(true);
}