// This will will be exported when using "cargo build" command
// It should create a my_library.dll (windows) file in the target/debug folder

/*
Add

[lib]
name = "my_library"
crate-type = ["dylib"]

to your cargo.toml to create the my_library.dll file when running "cargo build" command
*/

// https://github.com/alexcrichton/rust-ffi-examples/tree/master/python-to-rust
// https://doc.rust-lang.org/1.2.0/book/rust-inside-other-languages.html


// Testing and benchmark crate
#![feature(test)]
extern crate test;

// Exportable functions to be used inside Python and other languages
#[no_mangle]
pub extern fn double_input(input: i32) -> i32 {
    input * 2
}

#[no_mangle]
// Doesnt seem to work with u128
pub extern fn factorial(input: u64) -> u64 {
    if input <= 1 {
        return 1u64;
    }
    input * factorial(&input - 1)
}

#[cfg(test)] // Only compiles when running tests
mod tests {
    use super::*;
    use test::Bencher;

    // This will only be executed when using "cargo test" and not "cargo bench"
    #[test]
    fn test_double_input_function() {
        assert_eq!(4, double_input(2));
    }

    #[test]
    fn test_factorial_function() {
        assert_eq!(2, factorial(2));
        assert_eq!(6, factorial(3));
        assert_eq!(24, factorial(4));
    }
}