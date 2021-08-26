use std::collections::HashSet;

#[test]
fn types1() {
    let x = 5u8;

    let mut vec = Vec::new();

    vec.push(x);

    println!("{:?}", vec);
}

#[test]
fn types2() {

    let closure = |x, _y| x;
    let fns: [fn(i32, char) -> i32; 10] = [closure; 10];
    
    println!("{}", fns[0](5, 'c'));
}

fn requires_vec(x: Vec<String>) {
    for s in x {
        println!("{}", s);
    }
}

fn requires_hash_set(x: HashSet<String>) {
    for s in x {
        println!("{}", s);
    }
}

// a more complicated type inference example

#[test]
fn types3() {
    let original = ["hello", "world", "something", "hello", "foo"];
    let x = original
        .iter()
        .filter(|&&x| x != "something")
        .map(|x| x.to_string())
        .collect();

    requires_vec(x);

    let y = original
        .iter()
        .filter(|&&x| x != "something")
        .map(|x| x.to_string())
        .collect();

    requires_hash_set(y);
}


