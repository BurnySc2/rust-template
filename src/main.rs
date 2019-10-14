// Commands:
// rustup default nightly
// rustup update
// Install crates:
// cargo install crate_name
// Run test functions:
// cargo test
// Run benchmark functions:
// cargo bench

// Testing and benchmark crate
#![feature(test)]
extern crate test;

use std::collections::HashSet;
use std::collections::HashMap;

// Time measurement
//use std::time::{Duration, Instant};

fn main() {
    math_operations1();
    math_operations2();
    string_conversions();
    string_operations();
    for_loop_operations();
    vec_operations();
    hash_map_operations();
    struct_operations();
    os_operations();
}

fn square_of_u32(x: &u32) -> u32 {
    // Example function with return value
    x * x
}

fn math_operations1() {
    let a = 5;
    let b = 6;
    let c = a + b;
    let d = a * b;
    let e = a / b;
//    println!("c is {}, d is {}, e is {}", c, d, e);
    // Power of uint, int, float32, float64
    let f: u32 = (a as u32).pow(b);
    let g: i32 = i32::pow(a as i32, b);
    let h = 2.5f32.powi(4);
    let i = 4.9f64.powf(3.7);
//    println!("f is {}, g is {}, h is {}, i is {}", f, g, h, i);
    let x: i32 = 2;
    assert_eq!(x.pow(5), 32);
    assert_eq!(square_of_u32(&5), 25);
}

fn math_operations2() {
    // Absolute values
    assert_eq!((-10i32).abs(), 10);
    // Signum
    assert_eq!(10i32.signum(), 1);
    assert_eq!(0i32.signum(), 0);
    assert_eq!((-10i32).signum(), -1);
    // Min and max values of i32
    assert_eq!(i32::min_value(), -2147483648);
    assert_eq!(i32::max_value(), 2147483647);
    // Round float to nearest integers, and truncate to int
    let my_float: f32 = 5.5;
    let nearest_integer = my_float.round() as i32;
    let truncated_number = my_float as i32;
    assert_eq!(6, nearest_integer);
    assert_eq!(5, truncated_number);
    // Sum of integers
    let my_integers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let my_sum = my_integers.iter().sum::<i32>();
    assert_eq!(55, my_sum);

}


fn string_conversions() {
    // Convert int to string
    let x: u32 = 10;
    let my_string: String = x.to_string();
    // Convert string to int
    let my_int1 = my_string.parse::<i32>().unwrap();
    let my_int2: i32 = my_string.parse().unwrap();
    // Convert string to float
    let my_float = "1.5".parse::<f32>().unwrap();
    // Convert float to string with 2 decimal places
    let my_float2 = format!("{:.2}", 1.2345);
    assert_eq!("10", my_string);
    assert_eq!(10, my_int1);
    assert_eq!(10, my_int2);
    assert_eq!(1.5 as f32, my_float);
    assert_eq!("1.23", my_float2);

}

fn string_operations() {
    let mut hello = String::from("Hello, ");

    // Append character or string to string
    hello.push('w');
    hello.push_str("orld!");
    assert_eq!("Hello, world!", hello);
    assert!(hello.contains("Hello"));
    assert_eq!("Hello", hello[..5].to_string());
    assert_eq!("world!", hello[hello.len() - 6..].to_string());

    // Seperate by white space
    let chunks: Vec<_> = hello.split_whitespace().collect();
    for chunk in chunks.iter() {
//        println!("Chunk: {}", chunk);
    }

    // Format string
    let my_string = format!("x = {}, y = {y}", 10, y = 30);
    assert_eq!("x = 10, y = 30", my_string);

    // Concatenate a list of strings
    let words_vector = vec!["Hello", "World!"];
    let concatenated = words_vector.join(", ");
    assert_eq!("Hello, World!", concatenated);

}

fn for_loop_operations() {
    // Print "Hello" 2 times
    for _ in 0..2 {
//        println!("Hello");
    }
}

fn vec_operations() {
    // Create vector
    let mut items: Vec<u32> = vec![5, 6, 8];
    // Insert item to vector
    items.insert(2, 7);
    // Get index of item
    let my_index = items.iter().position(|x| *x == 8).unwrap();
    assert_eq!(3, my_index);
    assert_eq!(4, items.len());
//    println!("Index of value 8 in my vector is: {}", my_index);
    // Remove item at index
    items.remove(my_index);
    assert_eq!(3, items.len());
//    println!("My vector is now after removing item at index {}: {:?}", &my_index, &items);

    // Iterate over vector
    for x in (&items).iter() {
        let my_square = square_of_u32(x);
        assert_eq!(x * x, my_square);
//        println!("square of {} is {}", x, my_square);
    }

    // Iterate over vector with indices
    for (i, x) in (&items).iter().enumerate() {
//        println!("Item {} = {}", i, x);
    }

    // Check if a value is in vector
    let value = 6;
    let result_bool = (&items).iter().any(|v| v == &value);
    assert_eq!(true, result_bool);
//    println!("Value {} is in vector {:?}: {}", value, &items, result_bool);

    // Reverse vector
    let items2: Vec<_> = (&items).iter().rev().collect();
    assert_eq!(7, *items2[0]);
    assert_eq!(6, *items2[1]);
    assert_eq!(5, *items2[2]);
//    println!("My reversed items: {:?}", &items2);

}

fn hash_set_operations() {
    // Hash set that contains i32 values
    let my_set: HashSet<i32> = HashSet::new();
}

fn hash_map_operations() {
    // Create hashmap from array that contains string as key, i32 as value
    let my_map: HashMap<&str, i32> = [
        ("one", 1),
        ("two", 2),
    ].iter().cloned().collect();
//    println!("{:?}", my_map);

    // Loop over hashmap key and value
    for (key, val) in &my_map {
//        println!("Key={key}, Value={val}", key=key, val=val);
    }

    // Check map if contains key
    let key = "one";
    let does_contain_key = my_map.contains_key(&key);
    assert_eq!(true, does_contain_key);
//    println!("Key '{}' contained in map: {}", key, does_contain_key);

    // Check map if contains value
    let value = 2;
    let does_contain_value = my_map.values()
        .find(|&val| *val == value)
        .is_some();
    assert_eq!(true, does_contain_value);
//    println!("Value '{}' contained in map: {}", value, does_contain_value);
}




struct Point2d {
    x: f64,
    y: f64
}
impl Point2d {
    fn origin() -> Point2d {
        Point2d { x: 0.0, y: 0.0 }
    }
}

struct Rectangle {
    p1: Point2d,
    p2: Point2d
}

impl Rectangle {
    fn is_square(&self) -> bool {
        let Point2d { x: x1, y: y1 } = self.p1;
        let Point2d { x: x2, y: y2 } = self.p2;
        (x1 - x2).abs() == (y1 - y2).abs()
//        (self.p2.x - self.p1.x).abs() == (self.p2.y - self.p1.y).abs()
    }
    fn area(&self) -> f64 {
        let Point2d { x: x1, y: y1 } = self.p1;
        let Point2d { x: x2, y: y2 } = self.p2;
        ((x1 - x2) * (y1 - y2)).abs()
//        ((self.p2.x - self.p1.x) * (self.p2.y - self.p1.y)).abs()
    }
}

pub fn struct_operations() {
    let my_rect = Rectangle {
        p1: Point2d { x: 0.0, y: 0.0 },
        p2: Point2d { x: 6.0, y: 5.0 }
    };
    let my_square = Rectangle {
        p1: Point2d { x: 0.0, y: 0.0 },
        p2: Point2d { x: 5.0, y: 5.0 },
    };
    assert_eq!(30 as f64, my_rect.area());
    assert_eq!(25 as f64, my_square.area());
    assert_eq!(false, my_rect.is_square());
    assert_eq!(true, my_square.is_square());
}

fn os_operations() {
    // Exit program
    std::process::exit(0);
}


// Test and benchmark configuration to test if functions are working correctly, and to test the performance of functions

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn it_works() {
        assert_eq!(4, square_of_u32(&2));
    }

    #[bench]
    fn bench_math_operations1(b: &mut Bencher) {
        b.iter(|| math_operations1());
    }

    #[bench]
    fn bench_math_operations2(b: &mut Bencher) {
        b.iter(|| math_operations2());
    }

    #[bench]
    fn bench_string_conversions(b: &mut Bencher) {
        b.iter(|| string_conversions());
    }

    #[bench]
    fn bench_string_operations(b: &mut Bencher) {
        b.iter(|| string_operations());
    }

    #[bench]
    fn bench_for_loop_operations(b: &mut Bencher) {
        b.iter(|| for_loop_operations());
    }

    #[bench]
    fn bench_vec_operations(b: &mut Bencher) {
        b.iter(|| vec_operations());
    }

    #[bench]
    fn bench_hash_set_operations(b: &mut Bencher) {
        b.iter(|| hash_set_operations());
    }

    #[bench]
    fn bench_hash_map_operations(b: &mut Bencher) {
        b.iter(|| hash_map_operations());
    }

    #[bench]
    fn bench_struct_operations(b: &mut Bencher) {
        b.iter(|| struct_operations());
    }
}
