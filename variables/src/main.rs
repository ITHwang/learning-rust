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


    
   
}
