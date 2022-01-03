use std::panic;

pub fn assert_that_panics<F: FnOnce() -> R + panic::UnwindSafe, R>(f: F) {
    let result = catch_unwind_silent(f);

    assert!(result.is_err())
}

fn catch_unwind_silent<F: FnOnce() -> R + panic::UnwindSafe, R>(f: F) -> std::thread::Result<R> {
    let prev_hook = panic::take_hook();
    panic::set_hook(Box::new(|_| {}));
    let result = panic::catch_unwind(f);
    panic::set_hook(prev_hook);
    result
}

