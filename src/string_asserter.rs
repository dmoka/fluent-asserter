use super::*;

//TODO: add And

//TODO: S - rename to asserter?! if so, everywhere we need to rename
pub trait StrAssertions<T>
where
    T: Into<String> + Clone,
{
    fn contains(&self, expected: &str);
    fn starts_with(&self, expected_start: &str);
    fn ends_with(&self, expected_end: &str);
    fn is_empty(&self);
    fn is_not_empty(&self);
    fn has_length(&self, expected_length: usize);
    fn contains_all(&self, args: &[&str]); //TODO: S - rename
    fn contains_any(&self, args: &[&str]); //TODO: S - rename
}

impl<T> StrAssertions<T> for Asserter<T>
where
    T: Into<String> + Clone,
{
    fn contains(&self, expected: &str) {
        let string = self.value.clone().into();
        let is_present = string.contains(expected);

        if !is_present {
            panic!(
                "Expected {} to contain '{}', but it does not.",
                self.name, expected
            );
        }
    }
    fn starts_with(&self, expected_start: &str) {
        let string = self.value.clone().into();
        let starts_with = string.starts_with(expected_start);

        if !starts_with {
            panic!(
                "Expected {} to start with '{}', but it does not.",
                self.name, expected_start
            )
        }
    }

    fn ends_with(&self, expected_end: &str) {
        let string = self.value.clone().into();
        let ends_with = string.ends_with(expected_end);

        if !ends_with {
            panic!(
                "Expected {} to end with '{}', but it does not.",
                self.name, expected_end
            )
        }
    }

    fn is_empty(&self) {
        let string = self.value.clone().into();

        if !string.is_empty() {
            panic!("Expected {} to be empty, but it is not.", self.name)
        }
    }

    fn is_not_empty(&self) {
        let string = self.value.clone().into();

        if string.is_empty() {
            panic!("Expected {} to not be empty, but it is.", self.name)
        }
    }

    fn has_length(&self, expected_length: usize) {
        let string = self.value.clone().into();
        let len = string.len();

        if len != expected_length {
            panic!(
                "Expected {} to have length {}, but it has {}",
                self.name, expected_length, len
            );
        }
    }

    fn contains_all(&self, args: &[&str]) {
        //TODO: create ctor field value with string?
        let string = self.value.clone().into();
        let contains_all = args.iter().all(|&w| string.contains(&w));

        //TODO: add the words in the error message which are not present
        if !contains_all {
            panic!(
                "Expected {} '{}' to contain the strings {:?}, but it does not.",
                self.name, string, args
            );
        }
    }

    fn contains_any(&self, args: &[&str]) {
        let string = self.value.clone().into();
        let contains_any = args.iter().any(|&w| string.contains(&w));

        if !contains_any {
            panic!(
                "Expected {} '{}' to contain at least one of the strings {:?}, but it does not.",
                self.name, string, args
            );
        }
    }
}
