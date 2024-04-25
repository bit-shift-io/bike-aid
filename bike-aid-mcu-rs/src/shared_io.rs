use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, signal::Signal};
use esp_hal::gpio::IO;

pub struct SharedIO {
    pub io: IO,
}

impl SharedIO {
    pub fn new(io: IO) -> Self {
        SharedIO {
            io,
           // signal: Signal::new(),
        }
    }

    pub async fn perform_io_operation(&self) {
        // Perform I/O operations using the pins
        // For example, read from input pin and write to output pin
        //let input_value = self.io.pins.gpio10.degrade();

        // Notify that the operation is complete
        //self.signal.signal(());
    }
}