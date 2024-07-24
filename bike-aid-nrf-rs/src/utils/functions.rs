#![allow(unused)]

use defmt::*;
use heapless::{String, Vec};
use nrf_softdevice::ble::Uuid;
use num_traits::Num;

pub fn map<T : Num + Copy>(num: T, in_min: &T, in_max: &T, out_min: &T, out_max: &T) -> T {
    let out_delta = *out_max - *out_min;
    let in_delta = *in_max - *in_min;
    ((num - *in_min) / in_delta) * out_delta + *out_min
}

pub fn min<T: PartialOrd>(a: T, b: T) -> T {
    match a < b {
        true => a,
        false => b,
    }
}

pub fn max<T: PartialOrd>(a: T, b: T) -> T {
    match a > b {
        true => a,
        false => b,
    }
}

// bitshift a 16 bit number into two 8 bit numbers
pub fn shift_split_u16(short_16: i16) -> [u8; 2] {
    [(short_16 >> 8) as u8, (short_16 & 0xff) as u8]
}


pub fn hex_char_to_u8(c: char) -> u8 {
    match c {
        '0'..='9' => c as u8 - b'0',
        'a'..='f' => c as u8 - b'a' + 10,
        'A'..='F' => c as u8 - b'A' + 10,
        _ => {
            // Default value in case of error
            warn!("Invalid character: {}", c);
            0
        }
    }
}

pub fn string_to_uuid(input: &str) -> [u8; 16] {
    let mut bytes = [0u8; 16];
    let mut byte_index = 0;
    let mut high_nibble = 0;

    for (index, c) in input.chars().enumerate() {
        if index == 8 || index == 13 || index == 18 || index == 23 {
            if c != '-' {
                // Handle error case
                warn!("Invalid UUID format");
                return [0u8; 16];
            }
            continue;
        }

        let nibble = hex_char_to_u8(c);
        if index % 2 == 0 {
            high_nibble = nibble << 4;
        } else {
            bytes[byte_index] = high_nibble | nibble;
            byte_index += 1;
        }
    }

    bytes
}


pub fn print_bytes_array(bytes: &[u8; 16]) {
    info!("[");
    for (index, byte) in bytes.iter().enumerate() {
        if index != 0 {
            info!(", ");
        }
        info!("0x{:02x}", byte);
    }
    info!("]");
}

pub fn bytes_to_string(bytes: &[u8]) -> &str {
    let mut length = bytes.len();

    // Find the index of the first null character from the end of the byte array
    // trim the null chars and new line from the end of the string
    while length > 0 && (bytes[length - 1] == 0 || bytes[length - 1] == b'\n') {
        length -= 1;
    }

    core::str::from_utf8(&bytes[..length]).unwrap_or_default()
}


// copy a u8 slice into a heapless Vec with unknown size
pub fn copy_u8_slice(slice: &[u8]) -> Result<Vec<u8, 64>, ()> {
    let mut copied_vec = Vec::new();
    for &byte in slice {
        if copied_vec.push(byte).is_err() {
            let mut new_vec = copied_vec.into_iter().collect::<Vec<u8, 64>>();
            new_vec.push(byte).map_err(|_| ())?;
            copied_vec = new_vec;
        }
    }
    Ok(copied_vec)
}


pub fn str_to_array(input: &str) -> [u8; 32] {
    let mut byte_array = [0u8; 32]; // Initialize a byte array of length 64 with zeros

    // Copy the bytes from the input string to the byte array
    let input_bytes = input.as_bytes();
    let copy_length = input_bytes.len().min(32);
    byte_array[..copy_length].copy_from_slice(&input_bytes[..copy_length]);

    byte_array
}

pub fn bytes_to_array(input: &[u8]) -> [u8; 32] {
    let mut padded_array = [0u8; 32]; // Initialize a byte array of length 64 with zeros

    let input_len = input.len().min(32); // Get the minimum of input length and 64

    // Copy the input slice to the padded array
    padded_array[..input_len].copy_from_slice(&input[..input_len]);

    padded_array
}

pub fn trim_null_characters(bytes: &[u8; 32]) -> &[u8] {
    let mut length = bytes.len();

    // Find the index of the first null character from the end of the byte array
    while length > 0 && bytes[length - 1] == 0 {
        length -= 1;
    }

    &bytes[..length]
}

pub fn sigmoid_simple (x: f64) -> f64 {
    use num_traits::real::Real;
    1.0 / (1.0 + (-x).exp())
}

pub fn ease_into_speed_limit(current_speed: f64, speed_limit: f64, acceleration: f64, time_microseconds: f64, transition_duration_seconds: f64) -> f64 {
    let time = time_microseconds / (transition_duration_seconds * 1_000_000.0); // Convert seconds to microseconds
    let target_speed = speed_limit - (speed_limit - current_speed) * sigmoid_simple(-time);
    let new_speed = current_speed + acceleration * (time_microseconds / 1_000_000.0); // Convert microseconds to seconds
    
    if new_speed > target_speed {
        target_speed
    } else {
        new_speed
    }
}

pub fn sigmoid(x: f64, limit: f64, k: f64, x0: f64) -> f64 {
    use num_traits::real::Real;
    limit / ( 1.0 + (-k * (x - x0)).exp() )
}

pub fn hard_sigmoid(x: f64) -> f64 {
    let slope = 0.5;
    let offset = 0.5;
    
    let mut y = x * slope + offset;
    y = if y < 0.0 { 0.0 } else { if y > 1.0 { 1.0 } else { y } };
    
    y
}