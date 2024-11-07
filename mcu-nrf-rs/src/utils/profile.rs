#![allow(dead_code)]
use defmt::info;
use embassy_time::Instant;
use super::globals;

pub struct Profile {
    pub name: [u8; globals::BUFFER_LENGTH],
    name_len: usize,
    start: Option<Instant>,
    pub iterations: u8,
    count: u8,
}

impl Profile {
    pub fn new(name: &str) -> Self {
        let mut name_bytes: [u8; globals::BUFFER_LENGTH] = [0; globals::BUFFER_LENGTH];
        let name_len = name.len().min(globals::BUFFER_LENGTH);
        name_bytes[..name_len].copy_from_slice(&name.as_bytes()[..name_len]);
        Profile {
            name: name_bytes,
            name_len: name.len(),
            start: None,
            iterations: 0,
            count: 0,
        }
    }

    pub fn iterations(&mut self, iterations: u8) {
        self.iterations = iterations;
    }

    pub fn start(&mut self) {
        match self.start.is_some() {
            true => {
                self.count += 1;
            }
            false => { 
                self.start = Some(Instant::now()); 
            }
        }
        
    }

    pub fn stop(&mut self) {
        if self.count >= self.iterations {
            let duration = Instant::now().duration_since(self.start.unwrap());
            info!("{} profile in {} ms", self.name_as_string(), duration.as_millis());
            self.reset();
        }
    }

    pub fn reset(&mut self) {
        self.count = 0;
        self.start = None;
    }

    pub fn name_as_bytes(&self) -> &[u8] {
        &self.name[..self.name_len]
    }

    pub fn name_as_string(&self) -> &str {
        let bytes = self.name_as_bytes();
        core::str::from_utf8(bytes).unwrap()
    }
}