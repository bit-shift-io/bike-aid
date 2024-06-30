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