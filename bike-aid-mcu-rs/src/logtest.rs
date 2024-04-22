#![no_std]

use esp_println::print;

#[cfg(not(feature = "no-op"))]
#[macro_export]
macro_rules! println {
    ($($tts:tt)*) => {
        esp_println::println!("{}", inner!($($tts)*));
    }
}

pub(crate) use println;

/*
pub fn println () {
    esp_println::println!("Spawn Task Count: {}", 3);
}*/