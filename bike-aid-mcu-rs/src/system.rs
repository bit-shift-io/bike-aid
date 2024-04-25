
use critical_section::Mutex;
use esp_hal::peripherals::I2C0;
use esp_hal::timer::TimerGroup;
use esp_hal::{clock::ClockControl, i2c::I2C, peripherals::Peripherals};
use esp_hal::gpio::IO;
use esp_hal::prelude::_fugit_RateExtU32;
use esp_hal::{embassy, Async};
use esp_println::logger::init_logger;
use static_cell::StaticCell;
use esp_hal::prelude::_esp_hal_system_SystemExt;



//static SHARED_ASYNC_I2C : StaticCell<Mutex<I2C<I2C0, Async>>> = StaticCell::new();

pub struct System {
   // pub io: IO,
   // pub peripherals: Peripherals,
}

impl System {
    pub fn init() -> Self {
        init_logger(log::LevelFilter::Info); 
        log::info!("SYSTEM : init");
    
        let peripherals = Peripherals::take();
        let system = peripherals.SYSTEM.split();
        let clocks = ClockControl::boot_defaults(system.clock_control).freeze();
        let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
        let timg0 = TimerGroup::new_async(peripherals.TIMG0, &clocks);

        // embassy
        embassy::init(&clocks,timg0);
    

        // assign to struct
        Self {
         //   io: io,
         //   peripherals,
        }
    }

    pub fn init_i2c() {
        /*
        // I2C
        log::info!("I2C : init");

        // https://github.com/peterkrull/quad/blob/main/software/rusty-quad/src/main.rs
        // Configure and setup shared async I2C communication
    
        // Initialize and configure I2C0
        let mut i2c0 = I2C::new_async(
            peripherals.I2C0,
            io.pins.gpio3,
            io.pins.gpio2,
            100u32.kHz(),
            &clocks,
        );

        // shared i2c bus
        // SHARED_ASYNC_I2C.init(Mutex::new(i2c0));
        
    
        // Start Scan at Address 1 going up to 127
        for addr in 1..=127 {
            // Scan Address
            let res = i2c0.read(addr as u8, &mut [0]);
    
            // Check and Print Result
            match res {
                Ok(_) => log::info!("I2C Device Found at Address {}", addr as u8),
                Err(_) => {},
            }
        };

        */
    }
}