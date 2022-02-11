# Rust Workshop

This is a Rust workshop specifically focused on those with little to no experience in Rust, but with at least some
experience in systems programming either using C or C++.

## Getting Started

Installing Rust should hopefully be straight forward. To install Rust, just go to [rustup.rs](https://rustup.rs), and
follow the instructions. After downloading and executing the installer, the following packages should be available:

- the Rust compiler `rustc`
- the build tool `cargo`
- and other things like documentation

You can check the installation by opening your favorite terminal (PowerShell, cmd.exe, bash, xterm, etc.) and running
the command `cargo new hello-world`. By entering into the newly created directory `hello-world` followed by the
command `cargo run`, you should see the *Hello, world!* printed to your screen.

I personally use the Jetbrains CLion as the programming environment for Rust, but I can recommend also VS Code.

## Structure

The workshop is seperated into several parts with increasing complexity level.

### Part 1

This part consists of very basic tasks and showcases around Rust's memory safety, the borrow checker and FFI bindings.

* [Task 1 - Interfacing a legacy C++ library](part-1/task-1-interface-legacy-library/README.md)
* [Task 2 - Modifying shared data using threads](part-1/task-2-modify-shared-data-using-threads/README.md)
* [Task 3 - Use external Rust libraries](part-1/task-3-use-external-rust-libraries)

### Part 2

In this part we will create an app which listens on USB hotplug events and operates on an in-memory collection to keep
track of the connected USB devices.

[Let's go](part-2/usb-hotplug-listener/README.md)

### Part 3

Now we have learned to write applications dealing with event-based data, and it is time to condense our knowledge and
develop an API around our USB app.

[Let's go](part-3/usb-api/README.md)

## Additional Resources

* [The Book](https://doc.rust-lang.org/book/): _The_ resource for learning Rust
* [Too Many Linked Lists](https://rust-unofficial.github.io/too-many-lists/): A great way to push yourself to understand
  Rust's borrower checker and memory model
* [A Firehose of Rust](https://www.youtube.com/watch?v=IPmRDS0OSxM): Introduction of Rust's references, lifetime
  guaranties and move semantics compared to C++
* [Crates Repository](https://crates.io): The repository of all published crates
* [Docs of all Crates](https://docs.rs/): The online reference to Rust crates
