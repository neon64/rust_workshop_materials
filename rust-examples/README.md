# `rust-examples` crate

This folder contains a Rust "crate", named `rust-examples`.

Rust's "crates" are like packages in e.g.: Python, JavaScript.

You can create a new crate by running `cargo new my-crate-name` in the terminal.

## Running the examples

The easiest way to run these examples is with VS Code and the [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer) extension.

Once the extension is enabled/loaded, you simply click "run test" next
to each test case, and you can view the output in the embedded terminal window.

### In general

`cargo` is Rust's package manager and build tool.

Two `cargo` commands are particularly useful for Rust binaries:

    $ cargo build

builds the project.

And
    
    $ cargo run

runs a binary (will start executing from the `main()` function).

In this case, our `main()` is empty, so `cargo run` won't do much.