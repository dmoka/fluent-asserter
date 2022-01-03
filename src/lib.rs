pub struct Checker<T> {
    value : T
}

impl Checker<String> {

    pub fn new(value: String) -> Self {
        Checker {
            value
        }
    }

    pub fn is_equal_to(self, expected_value: &str) {
        println!("hello from is is equal method");
    }
}

#[macro_export]
macro_rules! assert_that {
    ($value:expr) => {
        Checker::new($value)
    };
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let checker = assert_that!(String::from("dani"));

        checker.is_equal_to("s")
    }
}
