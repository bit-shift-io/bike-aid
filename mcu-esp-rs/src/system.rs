

use embassy_sync::blocking_mutex::raw::{CriticalSectionRawMutex, NoopRawMutex};
use esp_hal::clock::Clocks;
use esp_hal::peripherals::I2C0;
use esp_hal::system::SystemParts;
use esp_hal::timer::TimerGroup;
use esp_hal::{clock::ClockControl, i2c::I2C, peripherals::Peripherals};
use esp_hal::gpio::{GpioPin, Output, OutputPin, Pin, Pins, IO};
use esp_hal::prelude::_fugit_RateExtU32;
use esp_hal::{embassy, Async};
use esp_println::logger::init_logger;
use static_cell::StaticCell;
use esp_hal::prelude::_esp_hal_system_SystemExt;
use embassy_sync::mutex::Mutex;
use embassy_embedded_hal::shared_bus::asynch::i2c::I2cDevice;



//static SHARED_ASYNC_I2C : StaticCell<Mutex<I2C<I2C0, Async>>> = StaticCell::new();
pub fn pin_output(pin : i8, level : bool) -> () {

    let peripherals = Peripherals::take();
    // let mutex: Mutex<CriticalSectionRawMutex, Peripherals> = Mutex::new(Peripherals::take());
    // let mut peripherals = mutex.lock().await;

     let system = peripherals.SYSTEM.split();
     let clocks = ClockControl::boot_defaults(system.clock_control).freeze();
     let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

     //io.pins.gpio0.enable_output()

}


pub struct System {
    //peripherals: &'static Peripherals
}

impl System {
    pub fn init() -> Self {
        init_logger(log::LevelFilter::Info); 
        log::info!("SYSTEM : init");
    
        let peripherals = Peripherals::take();
       // let mutex: Mutex<CriticalSectionRawMutex, Peripherals> = Mutex::new(Peripherals::take());
       // let mut peripherals = mutex.lock().await;

        let system = peripherals.SYSTEM.split();
        let clocks = ClockControl::boot_defaults(system.clock_control).freeze();
        //let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
        let timg0 = TimerGroup::new_async(peripherals.TIMG0, &clocks);

        // embassy
        embassy::init(&clocks,timg0);

        Self {
        //    peripherals: &Peripherals::take()
        }
    }

    pub fn init_i2c() {
        // I2C
        log::info!("I2C : init");

        let peripherals = Peripherals::take();
        let system = peripherals.SYSTEM.split();
        let clocks = ClockControl::boot_defaults(system.clock_control).freeze();
        let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
/*
        static I2C_BUS: StaticCell<Mutex<NoopRawMutex, Twim<TWISPI0>>> = StaticCell::new();
        let config = twim::Config::default();
        let i2c = Twim::new(p.TWISPI0, Irqs, p.P0_03, p.P0_04, config);
        let i2c_bus = Mutex::new(i2c);
        let i2c_bus = I2C_BUS.init(i2c_bus);

        
        // https://github.com/peterkrull/quad/blob/main/software/rusty-quad/src/main.rs
        // Configure and setup shared async I2C communication

        let mut sda = Output::new(3, Level::Low, OutputDrive::Standard);
        let mut scl = Output::new(2, Level::Low, OutputDrive::Standard);
    */
        // Initialize and configure I2C0
        let mut i2c0 = I2C::new_async(
            peripherals.I2C0,
            io.pins.gpio0,
            io.pins.gpio10,
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


    }

    /*
    fn get_led_output_pin() -> mut Output {
        let mut led = Output::new(pin, Level::Low, OutputDrive::Standard);
        return led
    }
    
    fn get_button_intput_pin() -> mut Input {
        let mut button = Input::new(p.P0_11, Pull::Up);
        return button
    }
     */
}