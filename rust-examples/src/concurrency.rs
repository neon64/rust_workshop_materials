use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[test]
fn no_shared_data() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}

#[test]
fn data_race() {
    let mut x = 5;

    let handle = thread::spawn(|| {
        for i in 1..10 {
            // uncomment to see what happens!
            // x += 1;
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        x += 1;
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();

    println!("Final x value was {}", x);
}

// A `crate` called crossbeam has this handy feature called "scoped threads"
// which allow us to mutably borrow variables on a different thread,
// as long as we make sure that thread has exited before we
// try and use that variable again.

#[test]
fn crossbeam_scope_example() {
    let mut x = 5;

    // note, this blocks on all child threads finishing, before returning
    crossbeam::scope(|s| {
        s.spawn(|_| {
            for i in 1..10 {
                // uncomment to see what happens!
                x += 1;
                println!("hi number {} from the spawned thread!", i);
                thread::sleep(Duration::from_millis(1));
            }
        });
    }).unwrap();

    for i in 1..5 {
        x += 1;
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    println!("Final x value was {}", x);
}

#[test]
fn locking_example() {
    let x = Arc::new(Mutex::new(5));
    let x2 = Arc::clone(&x);

    let handle = thread::spawn(move || {
        for i in 1..10 {
            {
                let mut guard = x2.lock().unwrap();
                *guard += 1;
            }
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..10 {
        let mut guard = x.lock().unwrap();
        *guard += 1;
        drop(guard);
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();

    println!("Final x value was {}", x.lock().unwrap());
}