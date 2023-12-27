/*
Rust is designed with a high degree of concern about the correctness of programs.
Rust does all the type checking and borrow checking, 
but can't check that this function will do precisely what we intend.
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]  // test attribute
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
    // passed, failed, ignored, measured, filtered out, `Doc-tests`

    // Tests fail when something in the test function panics.
    // Each test is run in a new thread, and when the main thread sees that a test thread has died, the test is marked as failed.

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        // If the value is `false`, the `assert!` macro calls `panic!`.
        assert!(larger.can_hold(&smaller));
    }
    
    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        // Note !: negate that result
        assert!(!smaller.can_hold(&larger));
    }

    // When we're sure what a value will be.
    #[test]
    fn it_adds_two() {
        // left, right
        assert_eq!(4, add_two(2));
    }
    
    // When we're not sure what a value will be, but we know what the value definitely shouldn't be.
    #[test]
    fn it_adds_two2() {
        // left, right
        assert_ne!(5, add_two(2));
    }

    // assert_eq! && assert_ne!: must implement the `PartialEq` and the `Debug` traits.

    #[test]
    fn greeting_contains_name() {
        let greeting = greeting("John");
        
        assert!(
            greeting.contains("John"),
            "Greeting should contain name, value was {}", // custom message
            greeting
        );
    }

    #[test]
    // check if out code handles error conditions correctly.
    // A `should_panic` test would pass even if the test panics for a different reason.
    // To make it more precise, we can add an optional `expected` argument.
    #[should_panic(expected = "Guess value must be between 1 and 100, got 101")] 
    fn greater_than_100() {
        Guess::new(101);
    }

    #[test]
    fn it_works_2() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            // the ? operator, assert!(value.is_err())
            Err(String::from("it failed"))
        }
    }
    
    // Todo...
    // Controlling How Tests Are Run

    /*
    1) Running Tests in Parallel or Consecutively
    2) Showing Function Output
    3) Running Single Tests
    4) Filtering to Run Multiple Tests
    5) Ignoring Some Tests
    */

    // Test Organization
    // ...
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello, {}!", name)
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}", value);
        }
        Guess { value }
    }
}
