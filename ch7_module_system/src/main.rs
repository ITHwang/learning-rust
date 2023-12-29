fn main() {
    /*
    1. Modules
    
    1) The module system: 
        packages(a cargo feature), 
        crates(a tree of modules), 
        modules(readability, easy-reuse, control the privacy)
        paths

    2) Concepts
    - Crate: the smallest amount of code that the Rust compiler considers at a time.
        - Binary crates: programs you can compile to an executable.
        - Library crates: functionality intended to be shared with multiple projects.
    - Crate root: a source file that the Rust compiler starts from.
    - Package: a bundle of one or more crates that provides a set of functionality.
        - can contain multiple binary crates but only one library crate.
        - must contain at least one crate.
    */
    
    /*
    2. Defining modules

    - Start from the crate root: `src/main.rs` or `src/lib.rs`
    - Declaring modules: src/{module_name}.rs
    - Declaring submodules: src/{module_name}/{submodule_name}.rs
    - the `use` keyword
        - shortcut(a symbolic link in the filesystem)
        - Specifying the parent module when calling the function makes it clear that the function isn't locally defined.
    - An absolute path is better, because we tend to move code independently.
    - The `super` keyword let us have fewer places to update code in the future.
    - The `as` keyword: We can specify `as` and a new local name(alias).
    - The `pub` keyword
        - In Rust, items in a parent module can't use the private items inside child modules.
        - But items in child modules can use the items in their ancestor modules.
        - The `pub` keyword exposes inner parts of child modules' code.
    - Public Enums: Unlike a struct, if we make an enum public, all of its variants are public.
    - The `pub use` keywords: We're bringing an item into scope, 
        but also making that item available for others to bring into their scope.("Re-exporting")
    - Packages with a Binary and a Library
        - The module tree should be defined in `src/lib.rs`.
        - The binary crate becomes a user who uses any public items in the library crate just like the public API.
    */
}