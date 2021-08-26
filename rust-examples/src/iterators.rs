

/// Closures are functions that can capture the enclosing environment.
/// For example, a closure that captures the x variable:
///
/// |val| val + x
///
/// The syntax and capabilities of closures make them very convenient for on the fly usage.
/// Calling a closure is exactly like calling a function.
/// However, both input and return types can be inferred and input variable names must be specified.
#[test]
fn closures1() {
    // here we need to give some type information to the argument `x`, in order for type inference to work out
    let double = |x: f64| x * 2.0;
    println!("{}", double(3.0));
}

#[test]
fn closures2() {
    // There are three important traits which closures (and named functions) implement.
    // Fn, FnMut, and FnOnce
    //
    // Fn(X) -> Y            may access captured variables [without mutation], implies FnMut (and thus FnOnce)
    // FnMut(X) -> Y         may mutate captured variables, implies FnOnce          
    // FnOnce(X) -> Y        may consume captured variables
    
    // This closure doesn't capture any environment, so implements Fn (and thus FnMut and FnOnce)
    let plus = |x: f64, y: f64| x + y;
    println!("plus(3.0, 6.5) = {}", plus(3.0, 6.5));

    // This closure needs immutable access to environment
    let amount = 5.0;
    let plus_fixed_amount = |x| plus(x, amount);
    println!("plus_fixed_amount(5.3) = {}", plus_fixed_amount(5.3));

    // This closure needs mutable access to environment
    let mut awesomeness = 0;
    println!("awesomeness = {}", awesomeness);
    let mut modify_captured_variable = || awesomeness += 1;
    modify_captured_variable();
    println!("awesomeness = {}", awesomeness);

    // This closure consumes a captured variable
    let mut x = String::from("hello");
    let consume_and_return_x = move |suffix| { x.push_str(suffix); x };
    let y = consume_and_return_x(", world");
    println!("y = {}", y);
    // trying to use `x` here would result in an error, as `x`
}


#[test]
fn iterators1() {
    let x = [1, 2, 3, 4, 7, 9, 2];

    // iterators in Rust are an example of a "zero-cost abstraction"
    for item in x.iter()
      .filter(|x| *x % 2 == 0)
      .map(|x| x * x) {
        println!("{}", item);
    }
}

use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;
use rayon::prelude::*;

/// This example uses the "rayon" library to create a list of random strings, and sort them in parallel.
/// (automatically splitting off into separate threads to make use of multicore CPUs - how cool!!)
/// https://rust-lang-nursery.github.io/rust-cookbook/concurrency/parallel.html
#[test]
fn parallel_iterators() {
    let mut vec = vec![String::new(); 100_000];
    vec.par_iter_mut().for_each(|p| {
        let mut rng = thread_rng();
        *p = (0..5).map(|_| rng.sample(&Alphanumeric)).map(char::from).collect()
    });
    vec.par_sort_unstable();
}