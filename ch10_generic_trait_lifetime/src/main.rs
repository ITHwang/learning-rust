fn main() {
    // 1. Generic types
    
    // generics: abstract stand-in for concrete types or other properties
    /*
    conventions: UpperCamelCase<T>
    */

    // 1.1. Function Definition
    // std::cmp::PartialOrd<T>: We can only use types whose values can be ordered.
    //                          We have to restrict the generic type.
    fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
        let mut largest = &list[0];

        for item in list {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);

    // 1.2. Struct Definition
    // struct Point<T, U> {
    //     x: T,
    //     y: U,
    // }
    // x and y are both generics but could have different types.
    // let integer_and_float = Point { x: 5, y: 10.0 };

    // 1.3. Enum Deinition
    // Option<T>, Result<T, E> in prelude.

    // 1.4. Method Definition
    struct Point<X1, Y1> {
        x: X1,
        y: Y1,
    }
    // Some generic parameters are declared with `impl` and
    // some are declared with the method definition.
    impl<X1, Y1> Point<X1, Y1> {
        fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
            Point { 
                x: self.x, 
                y: other.y,
            }
        }
    }

    let p1 = Point { x: 5, y: 10.0 };
    let p2 = Point { x: "hello", y: 'c' };
    let p3 = p1.mixup(p2);

    // 1.5. Performance of Generics
    /*
    1) Monomorphization is the process of turning generic code into specific code "when compiled".
    2) It performs just as it would if we had duplicated each definition by hand.
    3) The process of monomorphization makes Rust's generics extremely efficient "at runtime".
    */

    // 2. Traits: Defining Shared Behavior

    // 2.1. Trait Definition
    
    // Trait definitions are a way to group method signatures together to define a set of behaviors.
    pub trait Summary {
        fn summarize_author(&self) -> String;

        // Default Implementation
        fn summarize(&self) -> String {
            // Default Implementation can call other methods in the same trait.
            // The trait only require implementors to specify a small part of it.
            format!("(Read more from {}...", self.summarize_author())
        }
    }

    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }
    
    impl Summary for NewsArticle {
        fn summarize_author(&self) -> String {
            format!("#{}", self.author)
        }
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summary for Tweet {
        fn summarize_author(&self) -> String {
            format!("@{}", self.username)
        }
    }

    // To call the trait methods, the user must bring the trait into scope as well as the types.
    // use ch10_generic_trait_lifetime::{Summary, Tweet};

    // The Orphan Rule(part of `coherence`)
    /*
    We can implement a trait on a type only if at least one of the trait or the type is local to our crate.
    ex1. implement `Display` on `Tweet`(local)
    ex2. implement `Summary`(local) on `Vec<T>`

    But we can't implement external traits on external types.
    ex1. implement `Display`(external) on `Vec<T>`(external)

    Without th rule, two crates could implement the same trait for the same type, and Rust wouldn't know which implementation to use.
    */

    // 2.2. Trait as Parameters

    // A parameter `item` is a type that implements the `Summary` trait.
    pub fn notify1(item1: &impl Summary, item2: &impl Summary) {
        println!("Breaking news! {}", item1.summarize());
    }

    // Trait Bound
    pub fn notify2<T: Summary>(item1: &T, item2: &T) {
        println!("Breaking news! {}", item1.summarize());
    }
    
    // Multiple Trait Bounds
    
    use std::fmt::{Display, Debug};
    
    pub fn notify3(item1: &(impl Summary + Display)) {
        println!("Breaking news! {}", item1.summarize());
    }
    
    pub fn notify4<T: Summary + Display>(item1: &T) {
        println!("Breaking news! {}", item1.summarize());
    }

    // `where` Clauses
    
    // fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) {

    // fn some_function<T, U>(t: &T, u: &U) 
    // where
    //     T: Display + Clone,
    //     U: Clone + Debug,
    // {
    
    // Returning types that implement traits
    // returns some type that implements the `Summary` trait without naming the concrete type.
    fn returns_summarizable() -> impl Summary {          // a single type
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        }
    }

    // Blanket Implementations
    struct Pair<T> {
        x: T,
        y: T,
    }

    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Pair { x, y }
        }
    }

    // We can conditionally implement a trait for any type that implements another trait.
    impl<T: Display + PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("{} is greater than or equal to {}", self.x, self.y);
            } else {
                println!("{} is less than {}", self.x, self.y);
            }
        }
    }

    // Ex. The Standard Library
    // impl<T: Display> ToString for T {
    //     ...
    // }

    // Pros
    // 1. Reduce duplication
    // 2. Specify to the compiler that we want the generic type to have particular behavior.
    // 3. Fix the problems in compile time.

    // 3. Lifetimes

    // 3.1. Definition
    
    // Lifetime is the scope for which that reference is valid.
    // Rust requires us to annotate the relationships using generic lifetime parameters
    //     to ensure that actual references used at runtime will definitely be valid.
    // The main aim of lifetime is to prevent dangling references.

    // 3.2. The Borrow Checker

    // A borrow checker compares scopes to determine whether all borrows are valid.

    // let r;
    
    // {
    //     let x = 5;
    //     r = &x;
    // }
    
    // println!("r: {}", r);

    // 3.2. Syntax

    // &i32
    // &'a i32
    // &'a mut i32

    // 3.3. Lifetime Annotations in Function Signatures

    // Lifetime syntax is about connecting the lifetimes of various parameters and return values of functions.
    // The lifetime of the returned reference is the same as the smaller of the lifetimes of the values referred to by the arguments. 
    // Note: We're not changing the lifetimes of any values passed in or returned.
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    // When we pass concrete references to `longest`,
    //     the generic lifetime 'a will get the concrete lifetime that is equal to the smaller of the lifetimes of x and y.
    
    // let string1 = String::from("hello");
    // let result;
    // {
    //     let string2 = String::from("world!!!");
    //     result = longest(string1.as_str(), string2.as_str());
    // }
    // println!("The longest string is {}", result);

    // The lifetime parameter for the return type needs to match the lifetime parameter for one of the parameters.
    
    // fn longest_2<'a>(x: &str, y: &str) -> &'a str {
    //     let result = String::from("really long string");
    //     result.as_str()
    // }

    // 3.4. Lifetime Annotations in Struct Definitions

    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Couldn't find a '.'");
    // The instance of `ImportantExcerpt` can't outlive the reference it holds in its `part` field.
    let i = ImportantExcerpt { 
        part: first_sentence,
    };

    // 3.5. Lifetime Elision

    /*
    - The borrow checker could infer the lifetimes in these situations and wouldn't need explicit annotations.
    - If the compiler gets to the end of the three lifetime elision rules and there are still references for which it can't figure out lifetimes,
        the compiler will stop with an error.(`fn` and `impl`)

    1nd rule: the compiler assigns a lifetime parameter to each parameter that's a reference.
    2nd rule: if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters.
    3rd rule: if there are multiple input lifetime parameters, but one of them is `&self` or `&mut self` because this is a method,
              the lifetime of `self` is assigned to all output lifetime parameters.
    */
    
    /*
    ex1)
    fn first_word(s: &str) -> &str {   =>   PASS

    ex2)
    fn longest(x: &str, y: &str) -> &str {   =>   FAIL
    */

    // 3.6. Lifetime Annotations in Method Definitions

    /*
    - Lifetime names for struct fields always need to be declared
        after the `impl` keyword and then used after the struct's name.
    */
    impl<'a> ImportantExcerpt<'a> {
        fn level(&self) -> i32 {
            3
        }

        fn announce_and_return_part(&self, announcement: &str) -> &str {
            println!("Attention please: {}", announcement);
            self.part
        }
    }

    // 3.7. The Static Lifetime

    /*
    `'static`: the affected reference can live for the entire duration of the program.
    */
    let s: &'static str = "I have a static lifetime"; // stored in the program's binary.
    // In fact, the lifetime of all string literals is `'static`.

    // 3.8. Summary

    fn longest_with_an_announcement<'a, T> (
        x: &'a str,
        y: &'a str,
        ann: T,
    ) -> &'a str
    where
        T: Display,
    {
        println!("Announcement: {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
}
