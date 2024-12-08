/*
- rustup: CLI tools for managing rust versions and related tools.
- fil name convention: *_*.rs (ex. find_diagrams.rs)
- The first breakpoint is main.rs, such as C, Java, etc.
- rustfmt: default rust formatter
- indentation: 4 spaces
*/

fn main() {
    // Note: println! is not a function, but a rust macro(see chap19).
    // Ending with semicolon.
    println!("Hello, world!");
}

/*
- simple compile: `rustc main.rs` -> compling -> ./main(a binary file)

But we can easily compile, build, release using "Cargo" which is a rust build tool and a package manager.

- `cargo new {project name}
    - .git and .gitignore
    - src
        - main.rs
    - Cargo.toml(config file)

- `cargo build`
    - compiles the project
        - Cargo.lock: keeps track of versions of dependencies.
        - .target/debug: includes the binary executable file.
    - `--release`
        - .target/release

- `cargo run`
    - if it has a executable file and the file doesn't be changed, it directly runs the file.
    - else, it builds the project and runs.

- If we wanna only check if it can be compiled without building the project
    - `cargo check`
*/
