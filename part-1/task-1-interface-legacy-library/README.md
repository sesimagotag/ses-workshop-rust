# Interfacing a legacy C++ library

One of the many things to love about Rust is its easy-to-use foreign-function-interface (FFI) to nearly any codebase.

As an example of the powerful tooling provided to get started with legacy library code we will bind against a cmake
compiled static library written in C++.

First, prove the C++ sources compile and run:

```shell
$ cd vendor/legacy++
$ mkdir target
$ cmake ..
$ make
$ ./legacy
before
42
and after
a + b = 106.4
a - b = -91.6
a * b = 732.6
a / b = 0.0747475
```

To prove yourself about the successfully generated bindings go back to the project root and execute `cargo test`.

## The Challenge

Complete the [src/main.rs](src/main.rs) to have it output the same as the legacy app from above cmake build.

Verify the results with `cargo run`.