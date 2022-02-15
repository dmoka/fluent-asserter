use super::*;

impl<T> Asserter<Option<T>> where T : PartialEq + std::fmt::Display  {
    pub fn has_some(&self, value: T) {
        match &self.value {
            Some(val) => {
                if *val != value {
                    panic!("Expected '{}' to have value {}, but it has {}.",&self.name, value, val);
                }
            },
            None => panic!("Expected '{}' to have value {}, but it is None.",&self.name, value),
        }
    }

    pub fn has_none(&self) {
        match &self.value {
            None => {}
            Some(val) => panic!("Expected '{}' to be None, but it is Some({}).",&self.name, val)
        }
    }

}

//TODO: S - add this to tests folder
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_has_some_with_some() {
        let option = Option::Some(3);
        assert_that!(option).has_some(3);

        assert_that_code!(||assert_that!(option).has_some(4))
            .panics()
            .with_message("Expected 'option' to have value 4, but it has 3.");
    }

    #[test]
    fn test_has_some_with_none() {
        let option = Option::None;

        assert_that_code!(||assert_that!(option).has_some(4))
            .panics()
            .with_message("Expected 'option' to have value 4, but it is None.");
    }

    #[test]
    fn test_has_none_with_none() {
        let option = Option::<String>::None;
        assert_that!(option).has_none();
    }

    #[test]
    fn test_has_none_with_some() {
        let option = Option::Some(3);

        assert_that_code!(||assert_that!(option).has_none())
            .panics()
            .with_message("Expected 'option' to be None, but it is Some(3).");
    }

}