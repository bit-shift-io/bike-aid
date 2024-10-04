#![allow(unused)]

use heapless::String;
use num_traits::{Float, FromPrimitive, Num, One, ToPrimitive, Zero};

/// Maps an input value from one range to another.
///
/// ## Arguments
///
/// * `num` - The input number to be mapped.
/// * `in_min` - Minimum of the input range.
/// * `in_max` - Maximum of the input range.
/// * `out_min` - Minimum of the output range.
/// * `out_max` - Maximum of the output range.
///
/// ## Returns
///
/// The input value mapped to the output range, clamped to the input range.
pub fn map<T>(num: T, in_min: T, in_max: T, out_min: T, out_max: T) -> T
where
    T: Num + Copy + PartialOrd + Zero + One + ToPrimitive + FromPrimitive,
{
    // Clamp num to the input range
    let clamped_num = if num < in_min {
        in_min
    } else if num > in_max {
        in_max
    } else {
        num
    };

    let out_delta = out_max - out_min;
    let in_delta = in_max - in_min;

    // Convert to f64 to avoid overflow
    let mapped_value = (clamped_num.to_f64().unwrap() - in_min.to_f64().unwrap()) 
        * (out_delta.to_f64().unwrap() / in_delta.to_f64().unwrap()) 
        + out_min.to_f64().unwrap();

    // Convert back to the original type
    T::from_f64(mapped_value).unwrap() // Use from_f64 to convert back
}


/// Minimum of two numbers.
///
/// ## Arguments
///
/// * `a` - T number
/// * `b` - T number
///
/// ## Returns
///
/// T - number
pub fn min<T: PartialOrd>(a: T, b: T) -> T {
    match a < b {
        true => a,
        false => b,
    }
}

/// Maximum of two numbers.
///
/// ## Arguments
///
/// * `a` - T number
/// * `b` - T number
///
/// ## Returns
///
/// T - number
pub fn max<T: PartialOrd>(a: T, b: T) -> T {
    match a > b {
        true => a,
        false => b,
    }
}

/// Bitshift i16 to u8 array
///
/// ## Arguments
///
/// * `short_16` - i16
///
/// ## Returns
///
/// &[u8;2] byte array
pub fn bitshift_split_i16(value: i16) -> [u8; 2] {
    [(value >> 8) as u8, (value & 0xff) as u8]
}

/// Bitshift u16 to u8 array
///
/// ## Arguments
///
/// * `short_16` - i16
///
/// ## Returns
///
/// &[u8;2] byte array
pub fn bitshift_split_u16(value: u16) -> [u8; 2] {
    [(value >> 8) as u8, (value & 0xff) as u8]
}


/// Convert byte array to string
///
/// ## Arguments
///
/// * `&[u8]` - Byte array
///
/// ## Returns
///
/// &str
pub fn bytes_to_string(bytes: &[u8]) -> &str {
    let mut length = bytes.len();

    // Find the index of the first null character from the end of the byte array
    // trim the null chars and new line from the end of the string
    while length > 0 && (bytes[length - 1] == 0 || bytes[length - 1] == b'\n' || bytes[length - 1] == b'\r') {
        length -= 1;
    }

    core::str::from_utf8(&bytes[..length]).unwrap_or_default()
}

/// Convert byte array to heapless string
///
/// ## Arguments
///
/// * `&[u8]` - Byte array
///
/// ## Returns
///
/// heapless string <max size>
pub fn byte_array_to_heapless_string(byte_array: &[u8]) -> String<32> {
    let mut heapless_string: String<32> = String::new();

    for &byte in byte_array.iter() {
        if byte != 0 && byte != b'\n' && byte != b'\r'{ // Remove null and newline characters
            heapless_string.push(byte as char).unwrap();
        }
    }

    heapless_string
}