use super::*;

//TODO: make trait and implement it.
impl Asserter<bool> {
    /// Checks if the boolean is true.
    pub fn is_true(&self) {
        if !self.value {
            panic!("Expected {} to be true, but was false", self.name);
        }
    }

    /// Checks if the boolean is true.
    pub fn is_false(&self) {
        if self.value {
            panic!("Expected {} to be false, but was true", self.name);
        }
    }
}
