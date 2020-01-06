struct Point {
    x: i32,
    y: i32,
}

#[cfg(test)] // Only compiles when running tests
mod tests {
    use super::*;
    use test::Bencher;

    // This will only be executed when using "cargo test" and not "cargo bench"
    #[test]
    fn test_pattern_matching() {
        // https://doc.rust-lang.org/std/option/index.html

        enum Kingdom {
            Plant(u32, &'static str),
            Animal(u32, &'static str),
        }

        // A list of data to search through.
        let all_the_big_things = [
            Kingdom::Plant(250, "redwood"),
            Kingdom::Plant(230, "noble fir"),
            Kingdom::Plant(229, "sugar pine"),
            Kingdom::Animal(25, "blue whale"),
            Kingdom::Animal(19, "fin whale"),
            Kingdom::Animal(15, "north pacific right whale"),
        ];

        // We're going to search for the name of the biggest animal,
        // but to start with we've just got `None`.
        let mut name_of_biggest_animal = None;
        let mut size_of_biggest_animal = 0;
        for big_thing in &all_the_big_things {
            match *big_thing {
                Kingdom::Animal(size, name) if size > size_of_biggest_animal => {
                    // Now we've found the name of some big animal
                    size_of_biggest_animal = size;
                    name_of_biggest_animal = Some(name);
                }
                Kingdom::Animal(..) | Kingdom::Plant(..) => (),
            }
        }

        match name_of_biggest_animal {
            Some(name) => println!("the biggest animal is {}", name),
            None => println!("there are no animals :("),
        }
    }

    #[test]
    fn test_match() {
        let number = 13;
        // TODO ^ Try different values for `number`

        println!("Tell me about {}", number);
        match number {
            // Match a single value
            1 => println!("One!"),
            // Match several values
            2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
            // Match an inclusive range
            13..=19 => println!("A teen"),
            // Handle the rest of cases
            _ => println!("Ain't special"),
        }

        let boolean = true;
        // Match is an expression too
        let binary = match boolean {
            // The arms of a match must cover all the possible values
            false => 0,
            true => 1,
            // TODO ^ Try commenting out one of these arms
        };

        println!("{} -> {}", boolean, binary);
    }

    #[test]
    fn test_match2() {
        let p = Point { x: 0, y: 7 };

        match p {
            Point { x, y: 0 } => println!("On the x axis at {}", x),
            Point { x: 0, y } => println!("On the y axis at {}", y),
            Point { x, y } => println!("On neither axis: ({}, {})", x, y),
        }
    }
}
