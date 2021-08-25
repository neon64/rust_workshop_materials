use itertools::Itertools;
use std::borrow::Cow;

#[test]
fn fizzbuzz0() {
    for x in 1..11 {
        if x % 3 == 0 && x % 5 == 0 {
            println!("fizzbuzz")
        } else if x % 3 == 0 {
            println!("fizz")
        } else if x % 5 == 0 {
            println!("buzz")
        } else {
            println!("{}", x)
        }
    }
}

// Now let's try and use `match` iterators here...

fn fizzbuzz1_map(number: u32) -> String {
    match (number % 3, number % 5) {
        (0, 0) => "fizzbuzz".to_string(),
        (0, _) => "fizz".to_string(),
        (_, 0) => "buzz".to_string(),
        (_, _) => number.to_string()
    }
}

fn fizzbuzz1_join(max: u32, fun: &dyn Fn(u32) -> String) -> String {
    (0..=max).map(fun).join("\n")
}

#[test]
fn fizzbuzz1() {
    println!("{}", fizzbuzz1_join(10, &fizzbuzz1_map));
    println!("{}", (0..=10).map(&fizzbuzz1_map).join("\n"));
}

// Next, we try and make our function generic over a function type F

fn fizzbuzz2_join<F>(max: u32, fun: F) -> String where F: Fn(u32) -> String {
    (0..max).map(fun).join("\n")
}

#[test]
fn fizzbuzz2() {
    println!("{}", fizzbuzz2_join(10, fizzbuzz1_map));
}

// how can we make this more efficient?
// one option: no longer allocating a string for the constants fizz, buzz and fizzbuzz.
fn fizzbuzz3_map(number: u32) -> Cow<'static, str> {
    match (number % 3, number % 5) {
        (0, 0) => Cow::Borrowed("fizzbuzz"),
        (0, _) => Cow::Borrowed("fizz"),
        (_, 0) => Cow::Borrowed("buzz"),
        (_, _) => Cow::Owned(number.to_string())
    }
}

fn fizzbuzz3_join<'a, F>(max: u32, fun: F) -> String where F: Fn(u32) -> Cow<'a, str> {
    (0..max).map(fun).join("\n")
}

#[test]
fn fizzbuzz3() {
    println!("{}", fizzbuzz3_join(10, fizzbuzz3_map));
}