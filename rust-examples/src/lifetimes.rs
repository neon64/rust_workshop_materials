//! this example courtesy of:
//! https://depth-first.com/articles/2020/01/27/rust-ownership-by-example/

// this doesn't compile
// fn longest(x: &str, y: &str) -> &str {
//     if x.bytes().len() > y.bytes().len() {
//         x
//     } else {
//         y
//     }
// }

// we need explicit lifetime annotations here...
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.bytes().len() > y.bytes().len() {
        x
    } else {
        y
    }
}

#[test]
fn lifetimes1() {
    let alice = "Alice";
    let bob = "Bob";

    println!("{}", longest(alice, bob));
}

// this also doesn't compile!
// 
// #[derive(Debug)]
// struct Person {
//     name: &str // error: expected lifetime parameter
// }

#[derive(Debug)]
struct Person<'a> {
    name: &'a str
}