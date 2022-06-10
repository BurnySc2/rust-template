use rayon::prelude::*;

// https://crates.io/crates/rayon

fn sum_of_squares(input: &[i32]) -> i32 {
    input.par_iter().map(|&i| i * i).sum()
}

#[cfg(test)] // Only compiles when running tests
mod tests {
    use super::*;

    // This will only be executed when using "cargo test" and not "cargo bench"
    #[test]
    fn test_sum_of_squares() {
        let input = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let result = sum_of_squares(&input);
        assert_eq!(result, 385);
    }
}
