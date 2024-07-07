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
use embassy_nrf::{config::{DcdcConfig, Reg0Voltage}, gpio::{Level, Output, OutputDrive, Pin}, interrupt::{self, Priority}, peripherals::TWISPI0};
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
    /*
    // write REGOUT0 to set output voltage
    // should only need to do this once
    {
        pub enum Reg0Voltage {
            /// 1.8 V
            _1V8 = 0,
            /// 2.1 V
            _2V1 = 1,
            /// 2.4 V
            _2V4 = 2,
            /// 2.7 V
            _2V7 = 3,
            /// 3.0 V
            _3V0 = 4,
            /// 3.3 V
            _3v3 = 5,
            //ERASED = 7, means 1.8V
        }

        unsafe fn uicr_write_masked(address: *mut u32, value: u32, mask: u32) -> WriteResult {
            let curr_val = address.read_volatile();
            if curr_val & mask == value & mask {
                return WriteResult::Noop;
            }
        
            // We can only change `1` bits to `0` bits.
            if curr_val & value & mask != value & mask {
                return WriteResult::Failed;
            }
        
            let nvmc = &*pac::NVMC::ptr();
            nvmc.config.write(|w| w.wen().wen());
            while nvmc.ready.read().ready().is_busy() {}
            address.write_volatile(value | !mask);
            while nvmc.ready.read().ready().is_busy() {}
            nvmc.config.reset();
            while nvmc.ready.read().ready().is_busy() {}
        
            WriteResult::Written
        }

        unsafe {
            const UICR_REGOUT0: *mut u32 = 0x10001304 as *mut u32;
            let value = Reg0Voltage::_3v3 as u32;
            let res = uicr_write_masked(UICR_REGOUT0, value, 0b00000000_00000000_00000000_00000111);
            needs_reset |= res == WriteResult::Written;
            if res == WriteResult::Failed {
                warn!(
                    "Failed to set regulator voltage, as UICR is already programmed to some other setting, and can't be changed without erasing it.\n\
                    To fix this, erase UICR manually, for example using `probe-rs erase` or `nrfjprog --eraseuicr`."
                );
            }
        }
    }
    */
    // configure for softdevice
    // interrupt levels 0, 1 and 4 are reserved by the softdevice
    let mut config = embassy_nrf::config::Config::default();
    config.gpiote_interrupt_priority = Priority::P2;
    config.time_interrupt_priority = Priority::P2;
    config.dcdc.reg0 = true;
    config.dcdc.reg0_voltage = Some(Reg0Voltage::_3v3);
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
        p.P0_11.degrade()
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
    
    // pin 13 controls vcc output on/off 3.3v apparently?
    //Output::new(p.P0_13, Level::Low, OutputDrive::Standard);

    // loop for testing
    let pub_led = signals::LED_MODE.publisher().unwrap();
    let mut sub_minutes = signals::CLOCK_MINUTES.subscriber().unwrap();
    loop {
        let val = sub_minutes.next_message_pure().await;
        pub_led.publish_immediate(task_led::LedMode::OnOffSlow);
        info!("Clock: {:02}", val);
    }
 
}