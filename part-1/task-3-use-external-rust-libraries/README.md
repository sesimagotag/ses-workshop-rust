# Use external Rust libraries

Now we are going to integrate some non-Rust components with our Rust code. Rust components are so-called `crates` and
might contain binaries and/or libraries.

You might discover the vast amount of crates available at [crates.io](https://crates.io).

## The Challenge

We want to use the [libcurl](https://curl.se/libcurl/) library to perform an API request.

Fortunately, the [curl-rust](https://github.com/alexcrichton/curl-rust) project has already done the FFI bindings for us
in the crate `curl-sys`.

Finish the provided source to perform an API request to https://worldtimeapi.org/api/timezone/Europe/Vienna.