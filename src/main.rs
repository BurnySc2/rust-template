// Standard lib: ignore dead code and unused variables
#![allow(dead_code)]
#![allow(unused_variables)]
// Let clippy ignore single character variables
#![allow(clippy::many_single_char_names)]
// Testing and benchmark crate
#![feature(test)]
extern crate test;

// Import "cute" crate
#[macro_use(c)]
extern crate cute;

use std::collections::HashMap;
use std::collections::HashSet;

// Time measurement
//use std::time::{Duration, Instant};

fn square_of_u32(x: &u32) -> u32 {
    // Example function with return value
    x * x
}

fn math_operations1() {
    // Tuple unpacking
    let (a, b) = (5, 6);
    //    let (mut a, mut b) = (5, 6);
    let c = a + b;
    let d = a * b;
    let e = a / b;
    // Power of uint, int, float32, float64
    let f: u32 = (a as u32).pow(b);
    let g: i32 = i32::pow(a as i32, b);
    let h: f32 = 2.5f32.powi(4);
    let i: f64 = 4.9f64.powf(3.7);
    let x: i32 = 2;
    assert_eq!(x.pow(5), 32);
    assert_eq!(square_of_u32(&5), 25);
    assert_eq!(c, 11);
    assert_eq!(d, 30);
    assert_eq!(e, 0);
    assert_eq!(f, 15625);
    assert_eq!(g, 15625);
    assert!((h - 39.0625).abs() <= f32::EPSILON);
    assert!((i - 357.870_167_114_373_15).abs() <= f64::EPSILON);
}

fn math_operations2() {
    // Absolute values
    assert_eq!((-10i32).abs(), 10);
    // Signum
    assert_eq!(10i32.signum(), 1);
    assert_eq!(0i32.signum(), 0);
    assert_eq!((-10i32).signum(), -1);
    // Min and max values of i32
    assert_eq!(i32::MIN, -2_147_483_648);
    assert_eq!(i32::MAX, 2_147_483_647);
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
    assert!((1.5_f32 - my_float).abs() < f32::EPSILON);
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
    //    let b = vec![0, 1, 2];
    let b: Vec<i32> = c![x, for x in 0..3];
    for a in 0..3 {
        assert!(b.contains(&a));
    }

    // Continue and break statements
    for a in 0..5 {
        if a < 0 {
            continue;
        }
        if a > 2 {
            break;
        }
        assert!(b.contains(&a));
    }

    // Continue and break out of outer loop
    let mut a = -3;
    'my_label: loop {
        for c in a - 2..a + 2 {
            if a < 0 {
                a += 1;
                continue 'my_label;
            }
            if a > 2 {
                break 'my_label;
            }
        }
        assert!(b.contains(&a));
        a += 1;
    }
}

fn vec_operations() {
    // https://learning-rust.github.io/docs/b1.vectors.html
    // Create vector
    let a2: Vec<i32> = Vec::new();
    let b2: Vec<i32> = vec![];
    let b7 = vec![0; 10]; //Ten zeroes
    assert_eq!(10, b7.capacity());

    let mut items: Vec<u32> = vec![5, 6, 8];
    // Insert item to vector
    items.insert(2, 7);
    // Get index of item
    let my_index = items.iter().position(|x| *x == 8).unwrap();
    assert_eq!(3, my_index);
    assert_eq!(4, items.len());
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

    // Iterate over slice, does not work without the '&'
    let a = vec![1, 2, 3, 4, 5];
    for i in &a[1..4] {
        //        println!("{}", i);
    }

    // Iterate over 2 vecs at the same time, only loops until the smaller iter is exhausted
    let b = vec![1, 2, 3];
    let c = vec![3, 4, 5, 6];
    for (i, j) in b.iter().zip(c.iter()) {
        assert_eq!(*i + 2, *j);
    }

    // Iterate and do something with the value
    let d = vec![2, 4, 6];
    for (i, j) in b.iter().map(|x| 2 * x).zip(d.iter()) {
        assert_eq!(i, *j);
    }

    // Filter values
    for i in c.iter().filter(|&&x| x % 2 == 0) {
        //        println!("{:?}", i);
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

    // Convert array of arrays to vec of vecs
    let v = [[1, 2], [2, 3], [2, 3], [3, 4]];
    let v2 = v.iter().map(|x| x.to_vec()).collect::<Vec<Vec<i32>>>();
    assert_eq!(v2, vec![vec![1, 2], vec![2, 3], vec![2, 3], vec![3, 4]]);
}

fn hash_set_operations() {
    // Hash set that contains i32 values
    let mut my_set: HashSet<i32> = HashSet::new();

    // Insert a value
    my_set.insert(25);

    // Check if it contains a value
    assert!(my_set.contains(&25));

    // Remove a value and assert it is empty
    my_set.remove(&25);
    assert!(my_set.is_empty());

    // Add a value and assert that it is not empty
    my_set.insert(25);
    assert!(!my_set.is_empty());

    // Clear and asser that it is empty
    my_set.clear();
    assert!(my_set.is_empty());

    // Convert vec to hashset
    let v = vec![0, 0, 1, 2, 3, 4];
    let my_set2 = v.iter().cloned().collect::<HashSet<i32>>();
    assert_eq!(my_set2.len(), 5);
    let my_set3: HashSet<i32> = v.iter().cloned().collect();
    assert_eq!(my_set3.len(), 5);
    let my_set4: HashSet<i32> = v.clone().into_iter().collect();
    assert_eq!(my_set4.len(), 5);
    assert_eq!(v.len(), 6);
    // The following only works if 'v' is no longer used
    let my_set5: HashSet<i32> = v.into_iter().collect();
    assert_eq!(my_set5.len(), 5);
}

fn hash_map_operations() {
    // Create simple empty hash map
    let mut my_simple_map: HashMap<&str, i32> = HashMap::new();
    my_simple_map.insert("test", 5);
    my_simple_map.insert("test2", -5);
    let test_contained = my_simple_map.contains_key("test");
    assert!(test_contained);

    // Create hashmap from array that contains string as key, i32 as value
    let mut my_map: HashMap<&str, i32> = [("one", 1), ("two", 2)].iter().cloned().collect();
    //    println!("{:?}", my_map);

    // Add a key:value pair
    my_map.insert("added", 3);

    // Insert a value if it doesnt exist, and always update it
    *my_map.entry("added").or_insert(4) += 1;
    assert_eq!(my_map.get("added"), Some(&4));

    *my_map.entry("added2").or_insert(4) += 1;
    assert_eq!(my_map.get("added2"), Some(&5));
    *my_map.entry("added2").or_insert(4) += 1;
    assert_eq!(my_map.get("added2"), Some(&6));

    // Get value or insert default value and get that one if it didnt exist
    let value = my_map.entry("added3").or_insert(7);
    assert_eq!(*value, 7);
    *value += 1;
    assert_eq!(value, &8);
    assert_eq!(*value, 8);
    assert_eq!(my_map.get("added3"), Some(&8));

    // Insert a value if it doesnt exist
    my_map.entry("added4").or_insert(7);
    assert_eq!(my_map.get("added4"), Some(&7));

    // Remove a pair, returning its value. Returned value is an Option https://doc.rust-lang.org/std/option/index.html
    my_map.insert("removed", 4);
    let removed_value = my_map.remove("removed");
    assert_eq!(removed_value, Some(4));
    // Remove pair that does not exist, returning None
    let removed_non_existing = my_map.remove("does not exist");
    assert_eq!(removed_non_existing, None);

    // Loop over hashmap key and value
    for (key, val) in &my_map {
        //        println!("Key={}, Value={}", key, val);
    }

    // Check map if contains key
    let key = "one";
    let does_contain_key = my_map.contains_key(&key);
    assert!(does_contain_key);
    //    println!("Key '{}' contained in map: {}", key, does_contain_key);

    // Check map if contains value (shouldn't ever be used because it should be slow)
    let value = 2;
    //    let does_contain_value = my_map.values()
    //        .find(|&val| *val == value)
    //        .is_some();
    let does_contain_value = my_map.values().any(|val| *val == value);
    assert!(does_contain_value);
    //    println!("Value '{}' contained in map: {}", value, does_contain_value);

    // Merge 2 hashmaps
    let mut first: HashMap<&str, i32> = [("one", 1), ("two", 2)].iter().cloned().collect();
    let second: HashMap<&str, i32> = [("one", 1), ("three", 3)].iter().cloned().collect();
    assert_eq!(first.get("one"), Some(&1));
    for (key, val) in &second {
        *first.entry(key).or_insert(0) += val
    }
    assert_eq!(first.get("one"), Some(&2));
    assert_eq!(first.get("three"), Some(&3));
}

// Create vector using something similar to python "list comprehension"
// https://crates.io/crates/cute
fn vec_comprehension() {
    let v1: Vec<u32> = (0u32..9).collect();
    assert_eq!(v1, vec![0, 1, 2, 3, 4, 5, 6, 7, 8]);
    // All even numbers: [0, 2, 4, 6, 8]
    let v1: Vec<u32> = (0u32..9).filter(|x| x % 2 == 0).collect::<Vec<_>>();
    assert_eq!(v1, vec![0, 2, 4, 6, 8]);
    let v2: Vec<_> = c![x, for x in 0u32..9, if x % 2 == 0];
    assert_eq!(v2, vec![0, 2, 4, 6, 8]);

    // All squares of even numbers: [0, 4, 16, 36, 64]
    let v3: Vec<u32> = (0u32..9)
        .filter(|x| x % 2 == 0)
        .map(|x| x.pow(2))
        .collect::<Vec<_>>();
    assert_eq!(v3, vec![0, 4, 16, 36, 64]);
    let v4: Vec<u8> = c![x*x, for x in 0u8..9, if x % 2 == 0];
    assert_eq!(v4, vec![0, 4, 16, 36, 64]);

    // Nested comprehension
    let nested1: Vec<Vec<u32>> = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let flat: Vec<u32> = c![x, for x in y, for y in nested1];
    assert_eq!(flat, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);

    let nested2: Vec<Vec<u32>> = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let even_flat: Vec<u32> = c![x, for x in y, for y in nested2, if x % 2 == 0];
    assert_eq!(even_flat, vec![2, 4, 6, 8]);
}

fn hash_set_comprehension() {
    // Create hashset using "cute" macro to create vector, then converting it to hashset
    let even_numbers_vec = c![x, for x in 0u32..9, if x % 2 == 0];
    let even_numbers_hashset: HashSet<u32> = even_numbers_vec.into_iter().collect();
    let mut even_numbers_hashset2: HashSet<u32> = HashSet::new();
    even_numbers_hashset2.extend([0, 2, 4, 6, 8].iter());
    assert_eq!(even_numbers_hashset, even_numbers_hashset2);
}

fn hash_map_comprehension() {
    // Using "cute" macro, results in
    /*
    {
        0: 0,
        1: 1,
        2: 4,
        3: 9,
    }
    */
    let squares_hashmap: HashMap<u32, u32> = c! {key => key*key, for key in 0u32..4};
    let mut expected_squares_map: HashMap<u32, u32> = HashMap::new();
    expected_squares_map.insert(0, 0);
    expected_squares_map.insert(1, 1);
    expected_squares_map.insert(2, 4);
    expected_squares_map.insert(3, 9);
    assert_eq!(expected_squares_map, squares_hashmap);

    // Conditional hashmap comprehension
    let v: Vec<(&str, i32)> = vec![("one", 1), ("two", 2), ("three", 3)];
    let map = c! {key => val, for (key, val) in v, if val == 1 || val == 2};

    let mut expected: HashMap<&str, i32> = HashMap::new();
    expected.insert("one", 1);
    expected.insert("two", 2);

    assert_eq!(map, expected);
}

use std::cmp::Ordering;
use std::collections::BinaryHeap;

// https://doc.rust-lang.org/std/collections/binary_heap/
#[derive(Copy, Clone, Debug)]
struct State {
    score: f64,
    position: usize,
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Since f64 doesn't have 'cmp', you have to implement it yourself
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        if (other.score - self.score).abs() < f64::EPSILON {
            Ordering::Equal
        } else if other.score < self.score {
            // do 'Ordering::Greater' here if you want max heap
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}

impl Eq for State {}

fn heap_operations() {
    // With custom 'Ord' implementation this is now a min heap instead of max heap
    let mut heap = BinaryHeap::new();
    assert_eq!(heap.peek(), None);
    heap.push(State {
        score: 5.,
        position: 0,
    });
    heap.push(State {
        score: 3.,
        position: 2,
    });
    heap.push(State {
        score: 2.,
        position: 4,
    });
    heap.push(State {
        score: 1.,
        position: 7,
    });
    // println!("heap {:?}", heap);
    assert_eq!(heap.len(), 4);
    assert_eq!(
        heap.pop(),
        Some(State {
            score: 1.,
            position: 7
        })
    );
    assert_eq!(
        heap.pop(),
        Some(State {
            score: 2.,
            position: 4
        })
    );
    assert_eq!(
        heap.pop(),
        Some(State {
            score: 3.,
            position: 2
        })
    );
    assert_eq!(
        heap.pop(),
        Some(State {
            score: 5.,
            position: 0
        })
    );
    assert_eq!(heap.pop(), None);
    assert!(heap.is_empty());
    heap.clear();
    assert!(heap.is_empty());
}

// https://learning-rust.github.io/docs/b5.impls_and_traits.html
struct Point2d {
    x: f64,
    y: f64,
}

impl Point2d {
    fn origin() -> Point2d {
        Point2d { x: 0.0, y: 0.0 }
    }
    fn distance_to(&self, other: &Point2d) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
    fn distance_to_squared(&self, other: &Point2d) -> f64 {
        (self.x - other.x).powi(2) + (self.y - other.y).powi(2)
    }
}

struct Rectangle {
    p1: Point2d,
    p2: Point2d,
}

trait Square {
    fn is_square(&self) -> bool;
}
trait Area {
    fn area(&self) -> f64;
}

impl Square for Rectangle {
    fn is_square(&self) -> bool {
        let Point2d { x: x1, y: y1 } = self.p1;
        let Point2d { x: x2, y: y2 } = self.p2;
        ((x1 - x2).abs() - (y1 - y2).abs()).abs() < f64::EPSILON
        //        (self.p2.x - self.p1.x).abs() == (self.p2.y - self.p1.y).abs()
    }
}
impl Area for Rectangle {
    fn area(&self) -> f64 {
        let Point2d { x: x1, y: y1 } = self.p1;
        let Point2d { x: x2, y: y2 } = self.p2;
        ((x1 - x2) * (y1 - y2)).abs()
        //        ((self.p2.x - self.p1.x) * (self.p2.y - self.p1.y)).abs()
    }
}

fn struct_operations() {
    let my_rect = Rectangle {
        p1: Point2d { x: 0.0, y: 0.0 },
        p2: Point2d { x: 6.0, y: 5.0 },
    };
    let my_square = Rectangle {
        p1: Point2d::origin(),
        p2: Point2d { x: 5.0, y: 5.0 },
    };
    assert!((30f64 - my_rect.area()).abs() < f64::EPSILON);
    assert!((25f64 - my_square.area()).abs() < f64::EPSILON);
    assert_eq!(false, my_rect.is_square());
    assert_eq!(true, my_square.is_square());
}

fn point_operations() {
    let a = Point2d { x: 5.0, y: 5.0 };
    let b = Point2d { x: 8.0, y: 9.0 };
    let dist: f64 = a.distance_to(&b);
    let dist_squared: f64 = a.distance_to_squared(&b);
    assert!((dist - 25f64.sqrt()).abs() < f64::EPSILON);
    assert!((dist_squared - 25f64).abs() < f64::EPSILON);
}

// Import from subfolder phrases
mod datastructures;
mod stuff;
fn import_operations() {
    // https://learning-rust.github.io/docs/d3.modules.html#03-In-a-different-file-different-directory
    //    stuff::greetings::hello(); // You can call `hello()` directly from stuff
    datastructures::stack::simple_option_take_example();
}

fn os_operations() {
    println!("Exiting program.");
    // Exit program
    std::process::exit(0);
}

mod crates;

fn main() {
    math_operations1();
    math_operations2();
    string_conversions();
    string_operations();
    for_loop_operations();
    vec_operations();
    vec_comprehension();
    hash_set_operations();
    hash_set_comprehension();
    hash_map_operations();
    hash_map_comprehension();
    struct_operations();
    point_operations();
    import_operations();
    heap_operations();
    os_operations();
}

// Test and benchmark configuration to test if functions are working correctly, and to test the performance of functions

#[cfg(test)] // Only compiles when running tests
mod tests {
    use super::*;
    use test::Bencher;

    // This will only be executed when using "cargo test" and not "cargo bench"
    #[test]
    fn it_works() {
        assert_eq!(4, square_of_u32(&2));
    }

    // This will be executed when using "cargo test" as well as benchmarked when using "cargo bench"
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

    #[bench]
    fn bench_vec_comprehension(b: &mut Bencher) {
        b.iter(|| vec_comprehension());
    }

    #[bench]
    fn bench_hash_set_comprehension(b: &mut Bencher) {
        b.iter(|| hash_set_comprehension());
    }

    #[bench]
    fn bench_hash_map_comprehension(b: &mut Bencher) {
        b.iter(|| hash_map_comprehension());
    }

    #[bench]
    fn bench_point_operations(b: &mut Bencher) {
        b.iter(|| point_operations());
    }

    #[bench]
    fn bench_import_operations(b: &mut Bencher) {
        b.iter(|| import_operations());
    }

    #[bench]
    fn bench_heap_operations(b: &mut Bencher) {
        b.iter(|| heap_operations());
    }
}
