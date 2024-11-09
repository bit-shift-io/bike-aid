
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


pub fn init(twim: TWISPI0, sda: AnyPin, scl: AnyPin) -> &'static mut mutex::Mutex<ThreadModeRawMutex, Twim<'static, TWISPI0>> {
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


pub async fn device_available(i2c_bus: &'static mutex::Mutex<ThreadModeRawMutex, Twim<'static, TWISPI0>>, address: u8) -> bool {
    let mut i2c = I2cDevice::new(i2c_bus);
    let result = i2c.write(address, &[]).await;
    match result {
        Ok(_) => return true,
        Err(_) => return false,
    }
}


// pub fn get_i2c_bus() -> &'static mut Mutex<NoopRawMutex, RefCell<Twim<'static, TWISPI0>>> {
//     I2C_BUS.lock(|i2c_bus| i2c_bus.i2c_bus.as_mut().unwrap())
// }


// pub fn get_new_device() -> I2cDevice<'static, NoopRawMutex, Twim<'static, TWISPI0>> {
//     let i2c_bus = (*I2C_BUS.get_mut()).i2c_bus.expect("Not initialized");
//     let i2c_bus = I2C_BUS.lock(|i2c_bus| i2c_bus.i2c_bus.as_mut().unwrap());
//     let i2c_bus = I2C_BUS.get_mut().i2c_bus.expect("Not initialized");
//     I2cDevice::new(&i2c_bus)
// }



// fn init_i2c_bus(twim: TWISPI0, sda: AnyPin, scl: AnyPin) -> &'static mut Mutex<NoopRawMutex, RefCell<Twim<'static, TWISPI0>>> {
//     bind_interrupts!(struct Irqs {SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0 => twim::InterruptHandler<peripherals::TWISPI0>;});
//     interrupt::SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0.set_priority(interrupt::Priority::P3);
    
//     let config = twim::Config::default();
//     let i2c = Twim::new(twim, Irqs, sda, scl, config); // sda: p0.08, scl: p0.06
//     let i2c_bus = NoopMutex::new(RefCell::new(i2c));
    
//     static I2C_BUS: StaticCell<NoopMutex<RefCell<Twim<TWISPI0>>>> = StaticCell::new();
//     I2C_BUS.init(i2c_bus)
// }