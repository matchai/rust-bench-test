use std::{time, thread};

pub fn fibonacci(num: u32) -> u32 {
    thread::sleep(time::Duration::from_secs(1));

    match num {
        0 | 1 => 1,
        _ => fibonacci(num - 2) + fibonacci(num - 1)
    }
}
