
use core::cell::RefCell;
use embassy_nrf::gpio::AnyPin;
use embassy_nrf::interrupt::{self, InterruptExt};
use embassy_nrf::bind_interrupts;
use embassy_nrf::peripherals::{self, TWISPI0};
use embassy_sync::blocking_mutex::raw::{NoopRawMutex, ThreadModeRawMutex};
use embassy_nrf::twim::{self, Twim};
use embassy_sync::blocking_mutex::{Mutex, NoopMutex};
use static_cell::StaticCell;
use embassy_embedded_hal::shared_bus::blocking::i2c::I2cDevice;

pub static I2C_BUS: Mutex<ThreadModeRawMutex, I2cBus> = Mutex::new(I2cBus { i2c_bus: None });

struct I2cBus {
    i2c_bus: Option<Mutex<NoopRawMutex, RefCell<Twim<'static, TWISPI0>>>>,
}


pub fn init(twim: TWISPI0, sda: AnyPin, scl: AnyPin) -> &'static mut Mutex<NoopRawMutex, RefCell<Twim<'static, TWISPI0>>> {
    bind_interrupts!(struct Irqs {SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0 => twim::InterruptHandler<peripherals::TWISPI0>;});
    interrupt::SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0.set_priority(interrupt::Priority::P3);
    
    let config = twim::Config::default();
    let i2c = Twim::new(twim, Irqs, sda, scl, config); // sda: p0.08, scl: p0.06
    let i2c_bus = NoopMutex::new(RefCell::new(i2c));
    
    static I2C_BUS: StaticCell<NoopMutex<RefCell<Twim<TWISPI0>>>> = StaticCell::new();
    I2C_BUS.init(i2c_bus)
}


// pub fn get_i2c_bus() -> &'static mut Mutex<NoopRawMutex, RefCell<Twim<'static, TWISPI0>>> {
//     I2C_BUS.lock(|i2c_bus| i2c_bus.i2c_bus.as_mut().unwrap())
// }


pub fn get_new_device() -> I2cDevice<'static, NoopRawMutex, Twim<'static, TWISPI0>> {
    let i2c_bus = (*I2C_BUS.get_mut()).i2c_bus.expect("Not initialized");
    let i2c_bus = I2C_BUS.lock(|i2c_bus| i2c_bus.i2c_bus.as_mut().unwrap());
    let i2c_bus = I2C_BUS.get_mut().i2c_bus.expect("Not initialized");
    I2cDevice::new(&i2c_bus)
}



fn init_i2c_bus(twim: TWISPI0, sda: AnyPin, scl: AnyPin) -> &'static mut Mutex<NoopRawMutex, RefCell<Twim<'static, TWISPI0>>> {
    bind_interrupts!(struct Irqs {SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0 => twim::InterruptHandler<peripherals::TWISPI0>;});
    interrupt::SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0.set_priority(interrupt::Priority::P3);
    
    let config = twim::Config::default();
    let i2c = Twim::new(twim, Irqs, sda, scl, config); // sda: p0.08, scl: p0.06
    let i2c_bus = NoopMutex::new(RefCell::new(i2c));
    
    static I2C_BUS: StaticCell<NoopMutex<RefCell<Twim<TWISPI0>>>> = StaticCell::new();
    I2C_BUS.init(i2c_bus)
}