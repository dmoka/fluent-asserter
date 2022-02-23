use super::*;

//TODO: make trait and implement it.
impl Asserter<bool> {
    pub fn is_true(&self) {
        if !self.value {
            panic!("Expected {} to be true, but was false", self.name);
        }
    }

    pub fn is_false(&self) {
        if self.value {
            panic!("Expected {} to be false, but was true", self.name);
        }
    }
}
