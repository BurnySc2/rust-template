use rand::prelude::*;

// https://crates.io/crates/rand

#[cfg(test)] // Only compiles when running tests
mod tests {
    use super::*;
    use test::Bencher;

    // This will only be executed when using "cargo test" and not "cargo bench"
    #[test]
    fn random_float() {
        let mut rng = thread_rng();
        let x: f64 = rng.gen(); // random number in range [0, 1)
        assert_eq!(0.0 <= x, x < 1.0);
        let y: f64 = rng.gen_range(-10.0, 10.0);
        assert_eq!(-10.0 <= y, y < 10.0);
    }

    #[test]
    fn random_shuffle() {
        let mut rng = thread_rng();
        let mut nums = [1, 2, 3, 4, 5];
        nums.shuffle(&mut rng);
    }
}
