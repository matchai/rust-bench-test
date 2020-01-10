pub fn fibonacci(num: u32) -> u32 {
    match num {
        0 | 1 => 1,
        _ => fibonacci(num - 2) + fibonacci(num - 1)
    }
}
