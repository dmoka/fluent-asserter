use super::*;
use num::Unsigned;
use num::traits::Pow;
use num::{Integer, Signed, Float};

macro_rules! abs_diff_unsigned_eq {
    ($x:expr, $y:expr, $d:expr) => {
        if (if $x > $y {
            $x - $y 
        } else {
            $y - $x
        }) > $d {
            panic!("AssertionError: not equal within delta");
        }
    }
}

macro_rules! abs_diff_signed_eq {
    ($x:expr, $y:expr, $d:expr) => {
        if (($x - $y).abs() > $d) { panic!("AssertionError: not equal within delta"); }
    }
}

macro_rules! abs_diff {
    ($x:expr, $y:expr) => {
        ($x - $y).abs()
    }
}

impl<T> ApproximatelyEqual<T, UnsignedIntApproxEqual> for Asserter<T> where T : Unsigned + Integer {
    fn is_approx_equal_to(self, expected: T, delta: T) {
        abs_diff_unsigned_eq!(self.value,expected,delta);
    }
}

impl<T> ApproximatelyEqual<T, SignedIntApproxEqual> for Asserter<T> where T : Signed + Integer {
    fn is_approx_equal_to(self, expected: T, delta: T) {
        abs_diff_signed_eq!(self.value,expected,delta);
    }
}

//TODO: is it fine for f32, or we should create separate for that?
impl<T> ApproximatelyEqual<T,FloatApproxEqual> for Asserter<T> where T :Float + std::fmt::Display{
    fn is_approx_equal_to(self, expected: T, delta: T) {
        let rounder = 10f64.pow(get_length_of_rounder(delta));
        
        let diff = abs_diff!(self.value,expected);

        let diff_f64 = round(diff, rounder);
        let delta_f64 =  round(delta, rounder);

        if diff_f64 > delta_f64 {
            panic!("The number '{}' is not approximately equal to '{}' within delta '{}'",self.name,expected,delta)
        }
    }
}

//TODO: add abstraction for this, either in a new struct with new or some logic class
fn get_length_of_rounder<T>(delta: T) -> f64 where T: ToString {
    //TODO: Shall we handle error here?
    return delta.to_string()
        .split('.')
        .last()
        .unwrap()
        .len()
        .to_string()
        .parse()
        .unwrap();
}

fn round<T>(diff: T, rounder: f64) -> f64 where T: std::fmt::Display {
    let number : f64 = format!("{}",diff).parse().unwrap();
    let number: f64 = (number * rounder).round() / rounder;
    
    number
}

