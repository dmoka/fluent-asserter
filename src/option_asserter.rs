use super::*;

impl<T> Asserter<Option<T>> where T : PartialEq + std::fmt::Display  {
    pub fn is_some(&self) {
        match &self.value {
            None => {panic!("Expected '{}' to be Some(_), but found None", &self.name)}
            Some(_) => {}
        }

    }

    pub fn is_some_with_value(&self, value: T) {
        match &self.value {
            Some(val) => {
                if *val != value {
                    panic!("Expected '{}' to be Some({}), but it is Some({}).",&self.name, value, val);
                }
            },
            None => panic!("Expected '{}' to be Some({}), but it is None.",&self.name, value),
        }
    }

    pub fn is_none(&self) {
        match &self.value {
            None => {}
            Some(val) => panic!("Expected '{}' to be None, but it is Some({}).",&self.name, val)
        }
    }

}

//TODO: S - add this to tests folder
#[cfg(test)]
mod test_option_asserter {
    use super::*;

    #[test]
    fn test_is_some_without_value() {
        let option = Option::Some(3);
        assert_that!(option).is_some();

        let option = Option::<i32>::None;

        assert_that_code!(||assert_that!(option).is_some())
            .panics()
            .with_message("Expected 'option' to be Some(_), but found None");
    }

    #[test]
    fn test_is_some_with_some() {
        let option = Option::Some(3);
        assert_that!(option).is_some_with_value(3);

        assert_that_code!(||assert_that!(option).is_some_with_value(4))
            .panics()
            .with_message("Expected 'option' to be Some(4), but it is Some(3).");
    }

    #[test]
    fn test_is_some_with_none() {
        let option = Option::None;

        assert_that_code!(||assert_that!(option).is_some_with_value(4))
            .panics()
            .with_message("Expected 'option' to be Some(4), but it is None.");
    }

    #[test]
    fn test_is_none_with_none() {
        let option = Option::<String>::None;
        assert_that!(option).is_none();
    }

    #[test]
    fn test_is_none_with_some() {
        let option = Option::Some(3);

        assert_that_code!(||assert_that!(option).is_none())
            .panics()
            .with_message("Expected 'option' to be None, but it is Some(3).");
    }

}