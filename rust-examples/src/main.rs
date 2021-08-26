//! Some examples of various features/concepts in Rust!
//!
//! Please see each module for examples.

// Each of these modules are essentially a bunch of 'tests'.
// they are structured this way so that in VS Code you can
// hit "run test" and see each example run individually

#[cfg(test)]
mod fizzbuzz;
#[cfg(test)]
mod iterators;
#[cfg(test)]
mod ownership;
#[cfg(test)]
mod enums;
#[cfg(test)]
mod lifetimes;
#[cfg(test)]
mod type_inference;
#[cfg(test)]
mod concurrency;
#[cfg(test)]
mod traits;

fn main() {
    println!("Hello, world");
}
