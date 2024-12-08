fn main() {
    // Structs: fields, key:value pairs
    #[derive(Debug)]
    struct User {
        _active: bool,
        _username: String,
        email: String,
        _sign_in_count: u64,
    }

    // Instance
    // Note: Rust doesn't allow us to mark only certain fields as mutable.
    let mut user1 = User {
        email: String::from("zzizzonHwang@example.com"),
        _username: String::from("zzizzonHwang"),
        _active: true,
        _sign_in_count: 1,
    };
    user1.email = String::from("zzizzonHwang123@example.com"); // dot notation

    // The field init shorthand
    fn _build_user(email: String, _username: String) -> User {
        User {
            email,
            _username,
            _active: true,
            _sign_in_count: 1,
        }
    }

    // update
    // Note: the struct update syntax uses = like an assignment.
    let _user2 = User {
        email: String::from("zzizzonKang@example.com"),
        ..user1
    };
    // println!("{:?}", user1); // Throws an error.

    // Tuple structs
    // when you
    //    1. wanna give the whole tuple a name and make the tuple a different type from the other tuples.
    //    2. don't wanna give the each field a name.
    // Note: Each struct you define is its own type,
    //       even though the fields within the struct are might have the same types.
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    // let _black = Color(0, 0, 0);
    let white = Color(255, 255, 255);
    let origin = Point(0, 0, 0);

    println!("{} {} {}", origin.0, origin.1, origin.2); // indexing

    // println!("{} {}", black.0, white.2); // indexing
    let Color(red, green, blue) = white; // destructing

    println!("red: {}, green: {}, blue: {}", red, green, blue);

    // Unit-like structs
    // Unit-like structs are structs that have no fields.
    struct _AlwaysEqual; // We'll later implement this struct.

    // let subject = AlwaysEqual;

    // Fields as references
    // struct User2 {
    //     active: bool,
    //     username: &str,
    //     email: &str,
    //     sign_in_count: u64,
    // }

    // causes an error: lifetime specifier required
    // let user3 = User2 {
    //     active: true,
    //     username: "zzizzonHwang",
    //     email: "zzizzonHwang@example.com",
    //     sign_in_count: 1,
    // };

    // Example: Rectangle
    // the pros of structs
    //    1. Fields are related to each other.
    //    2. Names of the fields are descriptive.
    /*
    By default, the curly brackets tell `println!()` to use formatting known as `Display`.

    The primitive types like `i32` and `f64` implement `Display` for themselves.
    But structs don't have a provided implementation of `Display`.

    {:?} or {:#?} tells `println!()` to use formatting known as `Debug` instead of Display.
    #[derive(Debug)] helps explicitly opt in to make a functionality to bring out debugging info.
    */
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    // println!("{:#?}", rect1);

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }
    println!(
        "The area of the rectangle is {} square pixels.",
        // automatic referencing and dereferencing
        // the same as `(&rect1).area()`
        rect1.area()
    );

    impl Rectangle {
        fn square(size: u32) -> Self {
            Self {
                width: size,
                height: size,
            }
        }
    }
    println!(
        "A square rectangle of side length 5: {:#?}",
        Rectangle::square(5) // Note that `::` is used for both associated funcs and namespaces.
    );

    // `dbg!()` macro takes ownership of an expression and prints to `stderr`.
    // c.f. `println!()` macro takes a reference and prints to `stdout`.
    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale),
        height: 100,
    };
    dbg!(&rect2);
}
