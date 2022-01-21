use std::panic;

//TODO: add to test utils as only used for tests. We should not eat our own dog food
pub fn assert_that_panics<F: FnOnce() -> R + panic::UnwindSafe, R>(f: F) {
    let result = catch_unwind_silent(f);

    assert!(result.is_err())
}

pub fn assert_that_does_not_panic<F: FnOnce() -> R + panic::UnwindSafe, R>(f: F) {
    let result = catch_unwind_silent(f);

    assert!(result.is_ok())
}

fn catch_unwind_silent<F: FnOnce() -> R + panic::UnwindSafe, R>(f: F) -> std::thread::Result<R> {
    let prev_hook = panic::take_hook();
    panic::set_hook(Box::new(|_| {}));
    let result = panic::catch_unwind(f);
    panic::set_hook(prev_hook);
    result
}

#[cfg(test)]
mod test_panic_asserter_helper {

    use super::*;

    #[test]
    fn test_assert_that_panics() {
        assert_that_panics(|| panic!("error"));
        assert_that_panics(|| panic!("validation error"));
    }

    #[test]
    fn test_assert_that_does_not_panics() {
        assert_that_does_not_panic(|| println!("gm"));
        assert_that_does_not_panic(|| println!("WAGMI"));
    }
}
