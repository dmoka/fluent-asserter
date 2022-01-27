use super::*;

impl Asserter<bool> {
    pub fn is_true(&self) 
    {
        if !self.value {
            panic!("Expected {} to be true, but was false", self.name);//TODO: use proper error message
        }
    }

    pub fn is_false(&self) 
    {
        if self.value {
            panic!("Expected boolean to be false, but was true");//TODO: use proper error message
        }
    }
}

//TODO: S - add this to tests folder
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_true() {
        assert_that!(true).is_true();

        assert_that_code!(||assert_that!(false).is_true()).panics();
    }

    #[test]
    fn test_error_message () {
        let false_var = false;
        assert_that_code!(||assert_that!(false_var).is_true())
            .panics()
            .with_message("Expected false_var to be true, but was false");
    }

    #[test]
    fn test_is_false() {
        assert_that!(false).is_false();

        assert_that_code!(||assert_that!(true).is_false())
            .panics()
            .with_message("Expected boolean to be false, but was true");
    }

    
    #[test]
    fn test_is_equal_to() {
        assert_that!(false).is_equal_to(false);
        assert_that!(true).is_equal_to(true);

        assert_that_code!(||assert_that!(true).is_equal_to(false)).panics();
        assert_that_code!(||assert_that!(false).is_equal_to(true)).panics();
    }

}