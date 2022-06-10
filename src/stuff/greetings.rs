pub fn hello() {
    println!("Hello, world!");
}

#[cfg(test)] // Only compiles when running tests
mod tests {
    use super::*;

    #[test]
    fn bench_hello() {
        hello();
    }
}
