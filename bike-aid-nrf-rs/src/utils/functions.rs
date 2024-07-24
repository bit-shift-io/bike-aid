#![allow(unused)]

use num_traits::Num;

/// Map like c++
///
/// ## Arguments
///
/// * `num` - The input number
/// * `in_min` - Input number min range
/// * `in_max` - Input number max range
/// * `out_min` - Output number min range
/// * `out_max` - Output number max range
///
/// ## Returns
///
/// The input value mapped to the output range
pub fn map<T : Num + Copy>(num: T, in_min: &T, in_max: &T, out_min: &T, out_max: &T) -> T {
    let out_delta = *out_max - *out_min;
    let in_delta = *in_max - *in_min;
    ((num - *in_min) / in_delta) * out_delta + *out_min
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
pub fn shift_split_u16(short_16: i16) -> [u8; 2] {
    [(short_16 >> 8) as u8, (short_16 & 0xff) as u8]
}


/// Converts &str to 32 byte array
///
/// ## Arguments
///
/// * `&str` - string
///
/// ## Returns
///
/// &[u8;32] byte array
pub fn str_to_array(input: &str) -> [u8; 32] {
    let mut byte_array = [0u8; 32]; // Initialize a byte array of length 64 with zeros

    // Copy the bytes from the input string to the byte array
    let input_bytes = input.as_bytes();
    let copy_length = input_bytes.len().min(32);
    byte_array[..copy_length].copy_from_slice(&input_bytes[..copy_length]);

    byte_array
}

/// Converts byte array to 32 byte array
///
/// ## Arguments
///
/// * `&[u8]` - Byte array
///
/// ## Returns
///
/// &[u8;32] byte array
pub fn bytes_to_array(input: &[u8]) -> [u8; 32] {
    let mut padded_array = [0u8; 32]; // Initialize a byte array of length 64 with zeros

    let input_len = input.len().min(32); // Get the minimum of input length and 64

    // Copy the input slice to the padded array
    padded_array[..input_len].copy_from_slice(&input[..input_len]);

    padded_array
}

/// Trim null characters and new line characters from byte array.
///
/// ## Arguments
///
/// * `&[u8:32]` - Byte array
///
/// ## Returns
///
/// &[u8] byte array
pub fn trim_null_characters(bytes: &[u8; 32]) -> &[u8] {
    let mut length = bytes.len();

    // Find the index of the first null character from the end of the byte array
    while length > 0 && bytes[length - 1] == 0 {
        length -= 1;
    }

    &bytes[..length]
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