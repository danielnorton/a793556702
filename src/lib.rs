//! # a793556702
//! 
//! A very simple crate to test publishing to crates.io.

/// Says hello
/// 
/// # Example
/// ```
/// use a793556702::say_hello;
/// 
/// let message = say_hello("Fred");
/// assert_eq!("Hello, Fred!", message);
/// ```
pub fn say_hello(name: &str) -> String {

    return format!("Hello, {}!", name);
}