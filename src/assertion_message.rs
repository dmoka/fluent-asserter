use super::*;


pub struct AssertionFailureMessage {
    actual: String,
    expected: String
}

impl AssertionFailureMessage {
    pub fn new(actual: String, expected: String) -> AssertionFailureMessage{
        AssertionFailureMessage {
            actual,
            expected
        }
    }

    pub fn panic_message(&self) -> String {
        format!("AssertionFailure:\nThe expected value is different from the actual one\nExpected: {}\nActual: {}\n", self.expected, self.actual)
    }
}
//TODO: add color


//TODO: S - add this to tests folder
#[cfg(test)]
mod assertion {
    use super::*;    

    #[test]
    fn panic() {
        let failure = AssertionFailureMessage::new(
            String::from("test"), 
            String::from("test2"));

        let panic_message: String = failure.panic_message();

        assert_that!(panic_message).is_equal_to(String::from("AssertionFailure:\nThe expected value is different from the actual one\nExpected: test2\nActual: test\n"))
    }

}