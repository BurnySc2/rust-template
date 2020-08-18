# Rust-Template

This is my personal template to write Rust code. It contains various example functions and tasks that I may have to look up in the future.

# Rust commands

##### Update rust installation
```
rustup default nightly
rustup update
```

##### Downgrade to specific rust version
`rustup install 1.40.0`

##### Downgrade to specific nightly version at date
`rustup install nightly-YYYY-MM-DD`

Apply version:

`rustup override set nightly-YYYY-MM-DD`

##### Install package so that you can use the rust package manager
[cargo-edit](https://www.steadylearner.com/blog/read/How-to-install-Rust)

`cargo install cargo-edit`

##### Install clippy linter
[cargo clippy](https://github.com/rust-lang/rust-clippy)
```
rustup component add clippy
cargo clippy
```

##### Run automatic formatting of source files
[cargo fmt](https://github.com/rust-lang/rustfmt)

```
rustup component add rustfmt
cargo fmt
```

##### Install crates
`cargo add crate_name`

##### Remove crates
`cargo rm crate_name`

##### Run main.rs
`cargo run`

##### Run test functions (and benchmark functions once)
`cargo test`

##### Run benchmark functions
`cargo bench`

##### Check for errors and unused variables etc
`cargo check`

# Smaller binaries

See https://github.com/johnthagen/min-sized-rust

With `xargo` I was able to get my binary down to 90kb

`cargo install xargo`

`xargo build --release`
