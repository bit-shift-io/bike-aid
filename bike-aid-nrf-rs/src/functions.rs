#![allow(unused)]

use defmt::*;
use heapless::String;
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
    use core::str;
    unsafe {
        str::from_utf8_unchecked(bytes)
    }
}