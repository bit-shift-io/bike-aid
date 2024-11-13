#![allow(dead_code)]
use embassy_embedded_hal::shared_bus::asynch::i2c::I2cDevice;
use embassy_nrf::gpio::AnyPin;
use embassy_nrf::bind_interrupts;
use embassy_nrf::peripherals::{self, TWISPI0};
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_nrf::interrupt;
use embassy_nrf::interrupt::InterruptExt;
use embassy_nrf::twim::{self, Twim};
use embedded_hal_async::i2c::I2c;
use static_cell::StaticCell;
use embassy_sync::mutex;

static ASYNC_I2C_BUS: StaticCell<mutex::Mutex<ThreadModeRawMutex, Twim<TWISPI0>>> = StaticCell::new();
static mut I2C_BUS_PTR: *mut mutex::Mutex<ThreadModeRawMutex, Twim<TWISPI0>> = 0 as *mut _;


pub fn init(twim: TWISPI0, sda: AnyPin, scl: AnyPin) -> &'static mut mutex::Mutex<ThreadModeRawMutex, Twim<'static, TWISPI0>> {
    bind_interrupts!(struct Irqs {SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0 => twim::InterruptHandler<peripherals::TWISPI0>;});
    interrupt::SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0.set_priority(interrupt::Priority::P3);
    let config = twim::Config::default();
    let i2c = Twim::new(twim, Irqs, sda, scl, config);
    let i2c_bus = mutex::Mutex::<ThreadModeRawMutex, _>::new(i2c);
    let result: &mut mutex::Mutex<ThreadModeRawMutex, Twim<'_, TWISPI0>> = ASYNC_I2C_BUS.init(i2c_bus);
    unsafe { I2C_BUS_PTR = result as *mut _; }
    result
}


pub async fn device_available(i2c_bus: &'static mutex::Mutex<ThreadModeRawMutex, Twim<'static, TWISPI0>>, address: u8) -> bool {
    let mut i2c = I2cDevice::new(i2c_bus);
    let result = i2c.write(address, &[]).await;
    match result {
        Ok(_) => return true,
        Err(_) => return false,
    }
}


pub fn get_i2c_bus() -> &'static mut mutex::Mutex<ThreadModeRawMutex, Twim<'static, TWISPI0>> {
    unsafe { &mut *I2C_BUS_PTR }
}


pub fn get_new_device() -> I2cDevice<'static, ThreadModeRawMutex, Twim<'static, TWISPI0>> {
    I2cDevice::new(get_i2c_bus())
}