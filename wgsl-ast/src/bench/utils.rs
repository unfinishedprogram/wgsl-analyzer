use std::time::Duration;

pub fn time_call<T, F: FnOnce() -> T>(f: F) -> (T, Duration) {
    let start = std::time::Instant::now();
    let result = f();
    let end = std::time::Instant::now();
    (result, end - start)
}
