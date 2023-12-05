fn main() {
    /*
    must insert "mut", because we cannot assign twice to immutable variable
    */
    let mut x = 5;
    x = 6;

    // When we wanna get a constant which is always immutable,
    const MAX_POINTS: u32 = 100_000; // must insert mut, all capitals, underscores.

    // Variable shadowing
    // just create new variable in the view of the compiler as hiding the original variable
    /*
    That's even not to say that the original variable is guaranteed to still exist.
    Compiler optimizations can cause the original variable to be overwritten, especially if the original variable isn't accessed again.
    */
    let x = 5;
    let x = x + 1; 
    let x = x + 2;   
    
    // Even we can change type.
    let spaces = "   ";
    let spaces = spaces.len();

    /* 
    A scalar type represents a single value. Rust has four primary scalar types: 
        - integers
        - floating-point numbers
        - Booleans
        - characters.
    */

    // Integer Overflow
    // When you’re compiling in debug mode,
    //     Rust includes checks for integer overflow that cause your program to panic at runtime
    // When you’re compiling in release mode with the --release flag,
    //     Rust performs two’s complement wrapping.(ex. 256u8 -> 1u8)

    // numeric operations
    // must operate on the same type.
    // addition
    let sum = 5 + 10; // i32(default)
    // subtraction
    let difference = 95.5 - 4.3; // f64(default)
    // multiplication
    let product = 4 * 30;
    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1
    // remainder
    let remainder = 43 % 5;

    let f: bool = false; // one byte

    // Char type is four bytes in size and represents a Unicode Scalar Value.
    // Zero-width spaces are all valid char values.
    let z: char = 'ℤ';
    let heart_eyed_cat = '😻';

    // Compound types can group multiple values into one type.
    // Features: fixed length, saved on stack, different types(tuples), same types(arrays)
    //     - tuples
    //     - arrays
    
    let tup: (i32, f64, u8) = (500, 6.4, 1); // declaration, different types
    let (x, y, z) = tup; // destruction
    let five_hundred = tup.0; // indexing

    // `()`: Unit that represents an empty value or an empty return type.
    // Expressions implicitly return the unit value if they don’t return any other value.

    let months: [&str; 12] = ["January", "February", "March", "April", "May", "June", "July",
          "August", "September", "October", "November", "December"];
    
    let a = [3; 5]; // [3, 3, 3, 3, 3]
    // println!("{:?}", a);

    let first = a[0]; // indexing

    // Invalid Array Element Access
    // When you provide an incorrect index, invalid memory can be accessed. 
    use std::io;
    
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    // panicking
    // let element = a[index];
    // println!("The value of the element at index {index} is: {element}");
    
    

    
    
}
