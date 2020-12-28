//
// file: program.rs
// author: Michael Brockus
// gmail: <michaelbrockus@gmail.com>
//

//
// Greet the user
//
pub fn greet() -> &'static str
{
    "Hello, Rust Developer."
} // end of functions greet

//
// foundation of the program and related
// application logic must be implemented
// in the foundation.
//
pub fn foundation()
{
    println!("{}", greet());
} // end of function foundation
