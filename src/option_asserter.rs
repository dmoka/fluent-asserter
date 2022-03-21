use super::*;

//TODO: make trait and implement it

impl<T> Asserter<Option<T>>
where
    T: PartialEq + std::fmt::Display,
{
    pub fn is_some(&self) {
        match &self.value {
            None => {
                panic!("Expected '{}' to be Some(_), but found None", &self.name)
            }
            Some(_) => {}
        }
    }

    pub fn is_some_with_value(&self, value: T) {
        match &self.value {
            Some(val) => {
                if *val != value {
                    panic!(
                        "Expected '{}' to be Some({}), but found Some({}).",
                        &self.name, value, val
                    );
                }
            }
            None => panic!(
                "Expected '{}' to be Some({}), but found None.",
                &self.name, value
            ),
        }
    }

    pub fn is_none(&self) {
        match &self.value {
            None => {}
            Some(val) => panic!(
                "Expected '{}' to be None, but found Some({}).",
                &self.name, val
            ),
        }
    }
}
