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
}
