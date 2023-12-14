use std::error::Error;

// Box<dyn Error>: trait object which means "any kind of error"(Chap 17)
/*
When a main function returns `Result<(), E>`, 
exit with a value of 0 if main returns Ok.
exit with a nonzero value if main returns Err.
*/

// fn main() -> Result<(), Box<dyn Error>> {
fn main() {
    /*
    Rust groups errors into two categories:
        recoverable errors(ex. file not found) -> Result<T, E>
        unrecoverable errors(ex. accessing beyond the end of an array) -> panic!()
    */

    /*
    1. panic! with unrecoverable errors

    When a panic occurs, it will
        1) print a failure message,
        2) unwind or abort.
            unwind: walk back up the stack, clean up the data.
            abort: end the program without cleaning up the data.
        3) and quit.
    */

    // A backtrace is a set of all the functions that have been called to get to this point.
    // $RUST_BACKTRACE=1 cargo run
    // let v = vec![1, 2, 3];
    // v[99];

    /*
    2. Result with recoverable errors
    */
    use std::fs::File;
    use std::io::ErrorKind;

    // A normal way
    // let greeting_file_result = File::open("hello.txt");
    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {:?}", e),
    //         },
    //         other_error => panic!("Problem opening the file: {:?}", other_error),
    //     },
    // };

    // Using closers is more clear way.
    // let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap()
    //     } else {
    //         panic!("Problem opening the file: {:?}", error);
    //     }
    // });

    // unwrap
    // If the `Result` is the `Ok`, `unwrap` will return the value inside the `Ok`.
    // If `Err`, it will call the `panic!` macro.
    // let greeting_file = File::open("hello.txt").unwrap();

    // expect
    // Unlike `unwrap`, `expect` can provide a custom error message.
    // `expect` can give more context than `unwrap`.
    // let greeting_file = File::open("hello.txt").expect("Problem opening the file");

    /*
    3. Propagating errors(returning an error to the calling code)
    */
    use std::io::{self, Read};
    
    fn read_username_from_file() -> Result<String, io::Error> {
        let username_file_result = File::open("hello.txt");

        let mut username_file = match username_file_result {
            Ok(file) => file,
            Err(error) => return Err(error), // return the error to the calling code
        };

        let mut username = String::new();

        // finally return Ok or Err variants.
        match username_file.read_to_string(&mut username) {
            Ok(_) => Ok(username),
            Err(error) => Err(error),
        }
    }
    // When the function provides return values,
    // it's up to the calling code to decide what to do with those values.
    // So we propagate all the success or error information upward for it to handle appropriatly.

    // the `?` operator
    // if the function returns Ok, it will return the value inside the Ok.
    // if the function returns Err, it will return the error inside the Err.
    fn read_username_from_file2() -> Result<String, io::Error> {
        let mut username_file = File::open("hello.txt")?;
        let mut username = String::new();
        username_file.read_to_string(&mut username)?;
        Ok(username)
    }
    // Error values that have the `?` operator go through the `from` function, defined in the `From` trait.
    // When the `?` calls the `from` function, the error value is converted into the error type defined in the return type.
    // This is useful when a function returns one error type.

    // shorten by chaining method calls
    fn read_username_from_file3() -> Result<String, io::Error> {
        let mut username = String::new();
        File::open("hello.txt")?.read_to_string(&mut username)?;
        Ok(username)
    }
    // Already implemented
    use std::fs;
        
    fn read_username_from_file4() -> Result<String, io::Error> {
        fs::read_to_string("hello.txt")
    }

    // To use `?` in a function, the function must return a `Result` or  a `Option`.
    // And you can't mix and match `Result` and `Option`.
    fn last_char_of_first_line(text: &str) -> Option<char> {
        text.lines().next()?.chars().last()
    }

    // causes the error
    // let greeting_file = File::open("hello.txt")?;
    // Ok(())

    /*
    4. To panic! or Not to panic!
    */

    /*
    When to panic!
    - There's no way to recover.
    - A bad state(some assumption, guarantee, contract, or invariant has been broken)
    - A failure is unexpected.
    - An input of a function is invalid.
    */

    /*
    When not to panic! -> use `Result`
    - a good default choice when defining a function.
    - Let the calling code decide whether to panic.
    - A failure is expected.
    */

    /*
    When to use `unwrap` or `expect`
    - very handy when prototyping, before you're ready to decide how to handle errors.
    - when we ensure the `Result` will have an Ok value but the compiler can't understand why(must document the reason)
    */
    use std::net::IpAddr;
    let home: IpAddr = "127.0.0.1" // the compiler isn't smart enough to know that this is valid.
        .parse()
        .expect("Invalid IP address");

    // 4. Custom types for validation
    // Having lots of error check is verbose and annoying.
    // use Rust's type system to simplify the error check code.

    // loop {
    //     // --snip--
    //     let guess: i32 = match guess.trim().parse() {
    //         Ok(num) => num,
    //         Err(_) => continue,
    //     };
    //     if guess < 1 || guess > 100 {
    //         println!("Out of range!");
    //         continue;
    //     }

    //     match guess.cmp(&secret_number) {
    //         // --snip--
    //     }
    // }

    pub struct Guess {
        value: i32,
    }

    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1 || value > 100 {  // check whether to violate the contract
                panic!("Out of range: {}.", value);
            }

            Guess{ value }
        }

        pub fn value(&self) -> i32 { // self.value is private.
            self.value
        }
    }
    
}