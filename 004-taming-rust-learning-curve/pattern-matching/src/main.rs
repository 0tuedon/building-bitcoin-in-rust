fn main() {
    // first pattern matching with some
    let some_option = Some(42);
    if let Some(val) = some_option {
        println!("{}", val); // prints 42
    }
    // Rust global variables are called Statics
    // static N: i32 = 5;
    // static mut M: i32 = 15;
}
