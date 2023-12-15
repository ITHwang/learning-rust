fn main() {
    /*
    'Ownership' is a set of rules that govern how data is managed in Rust.
    */

    /*
    1. Stack and Heap
    - All data stored on the stack must have a known, fixed size.
    - Data with an unknown size at compile time or a size that might change must be stored on the heap.
    - Main purpose of ownership is to manage heap data as well as stack data.
    */

    /*
    2. Ownership Rules
    1) Each value in Rust has an owner.
    2) There can only be one owner as a time.
    3) When the owner goes out of scope, the value will be dropped.
    */

    /*
    3. Memory & Allocation
    - When a variable goes out of scope, Rust calls the `drop` function,
    - but in the case when variables of the primitive types go out of the scope, Rust drop the values without calling the `drop`.
    - The `drop` func is familar to 'Resource Acquisition Is Initialization'(RAII) in C++.
    - The values saved on the heap memory like String, is made up of three parts:
        1) A pointer to the memory that holds the contents
        2) A length
        3) A capacity
    */

    let s1 = String::from("hello");
    let s2 = s1; // moved to s2.(s1 is invalid.)

    let s1 = String::from("hello");
    let s2 = s1.clone(); // deepcopy

    // If a type implements the `Copy` trait, variables are not moved, but deep-copied.
    // The primitive types basically implement the `Copy` trait.

    /*
    4. References & Borrowing
    - Rust has a feature for using a value without transferring ownership. -> 'References'
    - A reference is like a pointer in that it's an address we can follow to access the data stored at that address.
    - Unlike a pointer, a reference is guaranteed to point to a valid value of a particular type.
    - A mutable reference allows us to modify a borrowed value.
    */

    /*
    Mutable References

    - If you have a mutable reference to a value, you can have no other references to that value.
    - This is for preventing data races at compile time.
    - Note: a reference's scope starts from where it is introduced
            and continues through the last time that reference is used.
    */

    /*
    Dangling References
    */

    /*
    5. The Slice Type
    */
}
