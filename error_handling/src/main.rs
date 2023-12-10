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
    let greeting_file = File::open("hello.txt").expect("Problem opening the file");

    /*
    3. Propagating errors
    */
    
}
