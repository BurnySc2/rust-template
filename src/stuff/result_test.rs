#[cfg(test)] // Only compiles when running tests
mod tests {
    use super::*;
    use test::Bencher;

    // This will only be executed when using "cargo test" and not "cargo bench"
    #[test]
    fn test_pattern_matching() {
        // https://doc.rust-lang.org/std/result/enum.Result.html

        let x: Result<i32, &str> = Ok(-3);
        assert_eq!(x.is_ok(), true);

        let x: Result<i32, &str> = Err("Some error message");
        assert_eq!(x.is_ok(), false);

        let x: Result<i32, &str> = Ok(-3);
        assert_eq!(x.is_err(), false);

        let x: Result<i32, &str> = Err("Some error message");
        assert_eq!(x.is_err(), true);

        // Convert Result to Option
        let x: Result<u32, &str> = Ok(2);
        assert_eq!(x.ok(), Some(2));

        let x: Result<u32, &str> = Err("Nothing here");
        assert_eq!(x.ok(), None);
    }
}
