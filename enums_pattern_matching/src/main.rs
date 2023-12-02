fn main() {
    // 1. Enumerations(enums)
    
    /*
    - Enums allows you to define a type by enumerating its possible "variants". 
    - An enum value can only be one of its variants.
    */
    enum IpAddrKind {
        V4,
        V6,
    }
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    /*
    - We can put data directly into each enum variant.
    - The name of each enum variant also becomes a constructor.
    - Each variant can have different types and amounts of associated data.
    */
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    // Another example
    struct QuitMessage;                        // unit struct 
    struct MoveMessage {                       // normal struct
        x: i32,
        y: i32,
    }
    struct WriteMessage(String);               // tuple struct
    struct ChangeColorMessage(i32, i32, i32);  // tuple struct

    // If we used the different structs,
    // we couldn't easily define a function to take these massages.
    // Instead,
    #[derive(Debug)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    
    // Also able to define methods on enums.
    impl Message {
        fn call(&self) {
            println!("{:#?}", self);
        }
    }
    let m = Message::Write(String::from("hello"));
    // m.call();

    // 2. Option

    /*
    - Option is an useful enum type which expresses that a value can be either something or nothing.
    - Using this type, the compiler can check whether you've handled all the cases, including when there is no value.
    - Unlike other languages, Rust doesn't use the null feature
        to avoid the bad case when you try to use a null value as a not-null value(ex. null pointer error).
    - Instead, Rust use "None" variants in Option type. 
    */
    // Already defined in the prelude.
    // enum Option<T> { 
    //     None,
    //     Some<T>,
    // }

    let x = Some(5);
    let y = Some('e');
    let absent_value: Option<i32> = None; // Must include a type of the None variant.

    // Why is having Option<T> any better than null?
    /*
    - Because Option<T> is able to check whether the value is None or not.
    - When we have a normal type, the compile will ensure that we always have a vaild value of the type.
    - When we have an Option type, the compile will check if the value is None or not before using that value.
    */

    // 3. match: A Control Flow Construct

    /*
    - In order to use an Option<T> value, you would wanna have code that will handle each variant.
    - Rust has a match expression that allows us to do that.
    */
    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
        // ...
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    // Unlike `if`, `match` can evaluate any type.
    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            // There are arms: patterns and some code.
            Coin::Penny => {
                println!("Lucky penny!");
                1
            } // the comma is optional.
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {  // binding value 
                println!("State quarter from {:?}!", state);
                25
            }
        }
        // The resultant value of the matched expression becomes the resultant value of the match expression.
    }

    // Note: Matches are exhaustive.    
    // The arms' patterns must cover all the possible cases.
    // causes an error.
    // just fixed it!
    // fn plus_one(x: Option<i32>) -> Option<i32> {
    //     match x {
    //         Some(i) => Some(i+1),
    //         None => Some(26),
    //     }
    // }

    let dice_roll = 9;
    let dice_roll2 = 9;
    
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        heeeng => move_player(heeeng), // catch-all patterns
    }

    match dice_roll2 {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(), // is a placeholder.
        // _ => (), // or do nothing.
    }

    fn add_fancy_hat() {
        println!("You got a fancy hat!");
    }

    fn remove_fancy_hat() {
        println!("You lost your fancy hat!");
    }

    fn move_player(num_spaces: u8) {
        println!("You moved! {}", num_spaces);
    }

    fn reroll() {
        println!("You rerolled!");
    }
                
}
