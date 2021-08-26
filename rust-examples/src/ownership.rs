
// Extract courtesy of:
// https://doc.rust-lang.org/stable/book/ch04-01-what-is-ownership.html
// 
// Ownership is Rust’s most unique feature, and it enables Rust to make
// memory safety guarantees without needing a garbage collector.
// Therefore, it’s important to understand how ownership works in Rust.
// In this chapter, we’ll talk about ownership as well as several related features:
// borrowing, slices, and how Rust lays data out in memory.

#[derive(Debug, Clone)]
struct Fizzler {
    amount: i32
}

// this allows us to print something out when MyType is 'dropped' (no longer needed / deallocated)
impl Drop for Fizzler {
    fn drop(&mut self) {
        println!("I'm being deallocated!");
    }
}

fn fizzle(m: Fizzler) {
    println!("Fizzling with amount={}", m.amount);
}

#[test]
fn ownership1() {
    let fizzler = Fizzler {
        amount: 3
    };

    // transfers ownership into `fizzle`
    fizzle(fizzler);

    // now cannot use `fizzler` anymore, ownership has been transferred into call to `fizzle`

    // uncommenting this will make the code not compile!
    //
    // println!("{}", fizzler.amount);
}

#[test]
fn ownership2() {
    let fizzler = Fizzler {
        amount: 3
    };

    fizzle(fizzler.clone());

    // now can still use `fizzler`, as we made a deep copy to pass into the function
    println!("{:?}", fizzler);
}

fn fizzle2(m: Fizzler) -> Fizzler {
    println!("Fizzling with amount={}", m.amount);

    // same as: `return m;`
    m 
}

#[test]
fn ownership3() {
    let fizzler = Fizzler {
        amount: 3
    };

    let fizzler = fizzle2(fizzler);

    // now can use `fizzler` again
    println!("{:?}", fizzler);
}

fn fizzle3(m: &Fizzler) {
    println!("Fizzling with amount={}", m.amount);
}

#[test]
fn ownership4() {
    let fizzler = Fizzler {
        amount: 3
    };

    // we use the `&` to create a **reference** to `fizzler`
    // (references are internallly implemented as pointers, but subject to additional rules)
    fizzle3(&fizzler);

    // now can still use `x`, as we got back ownership as the function returned...
    println!("We have the string again: {:?}", fizzler);
}

#[test]
fn iterator_invalidation() {
    let mut xs = Vec::new();
    xs.push(1);
    xs.push(2);
    xs.push(3);

    for x in xs.iter() {
        println!("{}", x);
        // Rust only allows one of ALIASING or MUTABILTY, not both at the same time.
        // This code would violate that constraint, hence doesn't compile.
        // xs.remove(0);
    }
}
