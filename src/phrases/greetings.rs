
pub fn hello() {
    println!("Hello, world!");
}

#[cfg(test)] // Only compiles when running tests
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_hello(b: &mut Bencher) {
        b.iter(|| hello());
    }
}

