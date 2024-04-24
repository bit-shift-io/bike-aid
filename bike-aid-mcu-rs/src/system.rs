use embassy_sync::mutex::Mutex;
use esp_hal::peripherals::I2C0;
use esp_hal::timer::TimerGroup;
use esp_hal::{clock::ClockControl, i2c::I2C, peripherals::Peripherals};
use esp_hal::gpio::IO;
use esp_hal::prelude::_fugit_RateExtU32;
use esp_hal::{embassy, Async};
use esp_println::logger::init_logger;
use static_cell::StaticCell;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex as CSRMutex;
use esp_hal::prelude::_esp_hal_system_SystemExt;


static SHARED_ASYNC_I2C : StaticCell<Mutex<CSRMutex, I2C<I2C0, Async>>> = StaticCell::new();

pub fn init () {
    // esp32 logger
    init_logger(log::LevelFilter::Info); 
    log::info!("SYSTEM : init");

    let peripherals = Peripherals::take();
    //let system = SystemControl::new(peripherals.SYSTEM);
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let timg0 = TimerGroup::new_async(peripherals.TIMG0, &clocks);

    embassy::init(&clocks,timg0);


    // I2C
    log::info!("I2C : init");

    // https://github.com/peterkrull/quad/blob/main/software/rusty-quad/src/main.rs
    // Configure and setup shared async I2C communication

    // Initialize and configure I2C0
    // let i2c0 = I2C::new_async <- async?
    let mut i2c0 = I2C::new_async(
        peripherals.I2C0,
        io.pins.gpio3,
        io.pins.gpio2,
        100u32.kHz(),
        &clocks,
    );

    // shared i2c bus
   // SHARED_ASYNC_I2C.init(Mutex::new(i2c0));

    /*
    // https://betterprogramming.pub/debugging-embedded-rust-e92ff0b8b8e5
    log::info!("Scanning I2C bus...\r");
    log::info!("     0  1  2  3  4  5  6  7  8  9  a  b  c  d  e  f\r");
    log::info!("00: "); // not new line

    // Loop over all addresses on the I2C bus
    for addr in 1..0xFF {
        // modulus, no remainder is end of the line
        if addr % 0x10 == 0 {
            log::info!("\r\n{:X}: ", addr); // not new line, but this simulates end of line
        }
        // We're issuing a simple scan to check if there's an ACK
        // We do not care about the result in the buffer but we need to
        // provide a non-empty one

        let res = i2c0.read(addr as u8, &mut [0]);

        match res {
            Ok(_) => {
                log::info!("{}", addr);
                log::info!("{:X} ", addr); // :X is hex uppercase
            }
            /*
            Err(hal::twim::Error::AddressNack) => {
                log::info!("-- ");
            } */
            Err(err) => {
                // Handle other error types if needed
                //log::info!("-- ");
                //log::info!("Error reading from TWIM: {:?}\r", err);
                //break;
            }
        }
    }
     */

    // Start Scan at Address 1 going up to 127
    for addr in 1..=127 {
        // Scan Address
        let res = i2c0.read(addr as u8, &mut [0]);

        // Check and Print Result
        match res {
            Ok(_) => log::info!("I2C Device Found at Address {}", addr as u8),
            Err(_) => {},
        }
    }
}