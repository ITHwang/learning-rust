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
    

    
}
