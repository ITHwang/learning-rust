// import the io library in standard library
/*
A "Prelude" refers to a set of standard modules that are automatically imported into every Rust program.
The prelude includes common traits, types, and functions that are frequently used.
When you don't have other traits in the prelude, you can import it.
*/
use std::io;
// Rng is kinda trait, which defines a set of methods that a type should implements(similar to interface or abstract class)
// Rng defines the set of random number generator's methods.
// To use the methods, we import Rng.
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess my number!");
    /*
    thread_rng(): returns a ThreadRng, which is a pseudo-random number generator.
    gen_range(): returns a value in the range of the given bounds.
    * Note: `cargo doc --open`: opens the docs about all dependencies we added in your browser.
    */
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Input an answer number you think");
        
        /*
        Declare a mutable variable
        Create a string instance(utf-8)
        Note `::`: associated function a.k.a. static method.
        */
        let mut guess = String::new();
        
        /*
        stdin(): return a standard input handler.
        
        read_line() gets a mutable reference of string instance.
        read_line() returns an io::Result instance which is kinda enums that have several variants.
        * Basically, Result type is for error handling.
    
        io::Result instance has two variants: Ok, Error
        io::Result::Ok(_) means success.
        io::Result::Err(_) means error.
    
        Regarding io::Result::expect(),
        If Result instance is a value of Err, 
            then it terminates the program and returns the error message.
        Else if it is a value of Ok,
            then it returns the value in the Ok instance.
            
        * If you don't use expect(), you get a warning message.
        */
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        /*
        Note: You can specify the type of the variable. 
              if you don't, Rust will infer the type.(default number type: i32)
        Variable shadowing: You can re-use the "guess" variable.
        trim(): remove "\n"
        parse(): convert string to number.
        
        */
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // _: represents all chars
        };
        
        println!("You guessed: {}", guess);

        /*
        When comparing guess to secret_number, 
            Rust will infer the type of secret_number as i32.

        Match expression are made up of several arms.
        Arms are made up of patterns and codes that executed when the patterns are matched.
        
        Match expression checks the patterns in order.
        When it meets the matched pattern, it executes following codes and is done.
        */
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
