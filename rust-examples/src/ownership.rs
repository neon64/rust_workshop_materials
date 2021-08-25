
// Extract courtesy of:
// https://doc.rust-lang.org/stable/book/ch04-01-what-is-ownership.html
// 
// Ownership is Rust’s most unique feature, and it enables Rust to make
// memory safety guarantees without needing a garbage collector.
// Therefore, it’s important to understand how ownership works in Rust.
// In this chapter, we’ll talk about ownership as well as several related features:
// borrowing, slices, and how Rust lays data out in memory.

fn do_something_with_string(mut x: String) {
    x.push_str(", world");
    println!("x = {}", x);
}

#[test]
fn ownership1() {
    // by default, most of Rust's simplest types (string literals, numbers) are copyable,
    // so we need to explicitly allocate memory for this string
    // in order to start caring about ownership etc...
    let x: String = "hello".to_owned();

    do_something_with_string(x);

    // now cannot use `x` anymore, ownership has been transferred into `do_something_with_string`

}

#[test]
fn ownership2() {
    let x: String = "hello".to_owned();

    do_something_with_string(x.clone());

    // now can still use `x`, as we made a deep copy to pass into the function
    println!("{}", x);
}

fn do_something_else_with_string(x: String) -> String {
    println!("I am doing something with string: {}", x);
    return x;
}

#[test]
fn ownership3() {
    let x: String = "hello".to_owned();

    let x = do_something_else_with_string(x);

    // now can still use `x`, as we got back ownership as the function returned...
    println!("We have the string again: {}", x);
}

fn do_something_else_with_borrowed_string(x: &String) {
    println!("I am doing something with string: {}", x);
}

#[test]
fn ownership4() {
    let x: String = "hello".to_owned();

    // we use the `&` to create a **reference** to x
    // (implemented as a pointer, but is subject to additional rules)
    do_something_else_with_borrowed_string(&x);

    // now can still use `x`, as we got back ownership as the function returned...
    println!("We have the string again: {}", x);
}

// hot tip: use &str instead of &String for added generality (i.e.: works with string literals as well)
fn do_something_with_str(x: &str) {
    println!("I am doing something with string: {}", x);
}

#[test]
fn ownership5() {
    let x: String = "hello".to_owned();

    // look ma! it works with both references to Strings, and with string literals (&'static str)
    do_something_with_str(&x);
    do_something_with_str("world");

}