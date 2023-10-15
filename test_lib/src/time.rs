use crate::prelude::*;

pub fn time_fn<F: Fn()>(count: u32, f: F) -> Duration {
    let start_time = Instant::now();
    for _ in 0..count {
        f();
    }
    let end_time = Instant::now();
    end_time - start_time
}
