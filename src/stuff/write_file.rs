static LOREM_IPSUM: &str =
    "Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod
tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam,
quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo
consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse
cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non
proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
";

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

// https://crates.io/crates/rayon

fn write_file() {
    let path = Path::new("src/stuff/lorem_ipsum.txt");
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
    match file.write_all(LOREM_IPSUM.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }
}

// use serde::;
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize, Debug)]
struct Address {
    street: String,
    city: String,
}

fn write_json() -> Result<()> {
    // Some data structure.
    let address = Address {
        street: "10 Downing Street".to_owned(),
        city: "London".to_owned(),
    };

    // Serialize object to a JSON string.
    let json_string = serde_json::to_string(&address)?;
    // Serialize indented https://stackoverflow.com/a/49087292/10882657
    //    let json_string = serde_json::to_string_pretty(&address)?;

    // Print, write to a file, or send to an HTTP server.
    println!("{}", json_string);

    let path = Path::new("src/stuff/json_test.json");
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    // Write the `` string to `file`, returns `io::Result<()>`
    match file.write_all(json_string.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }

    Ok(())
}

#[cfg(test)] // Only compiles when running tests
mod tests {
    use super::*;

    // This will only be executed when using "cargo test" and not "cargo bench"
    #[test]
    fn test_write_file() {
        write_file();
    }

    #[test]
    fn test_write_json() {
        let _ = write_json();
    }
}
