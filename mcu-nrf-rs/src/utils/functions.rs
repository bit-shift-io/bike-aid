use num_traits::{FromPrimitive, Num, One, ToPrimitive, Zero};

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