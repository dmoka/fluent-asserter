use super::*;
use num::traits::Pow;

pub trait IsApproxEqual<T> {
    fn is_approx_equal(&self, expected_value: T, delta: T);
}

macro_rules! abs_diff_unsigned_eq {
    ($x:expr, $y:expr, $d:expr) => {
        if (if $x > $y { $x - $y } else { $y - $x }) > $d {
            panic!("AssertionError: not equal within delta"); //TODO: improve error message
        }
    };
}

macro_rules! abs_diff_signed_eq {
    ($x:expr, $y:expr, $d:expr) => {
        if (($x - $y).abs() > $d) {
            panic!("AssertionError: not equal within delta"); //TODO: improve error message
        }
    };
}

macro_rules! abs_diff {
    ($x:expr, $y:expr) => {
        ($x - $y).abs()
    };
}

macro_rules! generate_is_approx_equal_impl_for_signed {
    ($TStructType:ident) => {
        impl IsApproxEqual<$TStructType> for Asserter<$TStructType> {
            fn is_approx_equal(&self, expected: $TStructType, delta: $TStructType) {
                abs_diff_signed_eq!(self.value, expected, delta);
            }
        }
    };
}

generate_is_approx_equal_impl_for_signed!(i8);
generate_is_approx_equal_impl_for_signed!(i16);
generate_is_approx_equal_impl_for_signed!(i32);
generate_is_approx_equal_impl_for_signed!(i64);
generate_is_approx_equal_impl_for_signed!(i128);

macro_rules! generate_is_approx_equal_impl_for_unsigned {
    ($TStructType:ident) => {
        impl IsApproxEqual<$TStructType> for Asserter<$TStructType> {
            fn is_approx_equal(&self, expected: $TStructType, delta: $TStructType) {
                abs_diff_unsigned_eq!(self.value, expected, delta);
            }
        }
    };
}

generate_is_approx_equal_impl_for_unsigned!(u8);
generate_is_approx_equal_impl_for_unsigned!(u16);
generate_is_approx_equal_impl_for_unsigned!(u32);
generate_is_approx_equal_impl_for_unsigned!(u64);
generate_is_approx_equal_impl_for_unsigned!(u128);

macro_rules! get_length_of_rounder {
    ($delta:expr) => {
        $delta
            .to_string()
            .split('.')
            .last()
            .unwrap()
            .len()
            .to_string()
            .parse()
            .unwrap()
    };
}

macro_rules! round {
    ($TStructType:ident, $diff:expr,$rounder:expr) => {{
        let number: $TStructType = format!("{}", $diff).parse().unwrap();
        let number: $TStructType = (number * $rounder).round() / $rounder;

        return number;
    }};
}

impl IsApproxEqual<f64> for Asserter<f64> {
    fn is_approx_equal(&self, expected_value: f64, delta: f64) {
        let rounder = 10f64.pow(get_length_of_rounder_f64(delta));

        let diff = abs_diff!(self.value, expected_value);

        let diff_f64 = round_f64(diff, rounder);
        let delta_f64 = round_f64(delta, rounder);

        if diff_f64 > delta_f64 {
            panic!(
                "The number '{}' is not approximately equal to '{}' within delta '{}'",
                self.name, expected_value, delta
            )
        }
    }
}

impl IsApproxEqual<f32> for Asserter<f32> {
    fn is_approx_equal(&self, expected_value: f32, delta: f32) {
        let rounder = 10f32.pow(get_length_of_rounder_f32(delta));

        let diff = abs_diff!(self.value, expected_value);

        let diff_f32 = round_f32(diff, rounder);
        let delta_f32 = round_f32(delta, rounder);

        if diff_f32 > delta_f32 {
            panic!(
                "The number '{}' is not approximately equal to '{}' within delta '{}'",
                self.name, expected_value, delta
            )
        }
    }
}

fn get_length_of_rounder_f64<T>(delta: T) -> f64
where
    T: ToString,
{
    return get_length_of_rounder!(delta);
}

fn get_length_of_rounder_f32<T>(delta: T) -> f32
where
    T: ToString,
{
    return get_length_of_rounder!(delta);
}

fn round_f64<T>(diff: T, rounder: f64) -> f64
where
    T: std::fmt::Display,
{
    round!(f64, diff, rounder)
}

fn round_f32<T>(diff: T, rounder: f32) -> f32
where
    T: std::fmt::Display,
{
    round!(f32, diff, rounder)
}
