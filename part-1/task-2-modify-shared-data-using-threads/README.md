# Modifying shared data through multiple threads

Rust ensures that our code is free from data races. How that mechanism works and how we encounter it on our daily
programming tasks is shown in this task.

## The Challenge

We want to have a shared counter variable altered by multiple threads. The prepared
`ThreadPool` type should be used to complete the challenge.

Verify your solution by executing `cargo test` in the project root.

As a bonus, alter the `ThreadPool` to support gracefully shut down all active workers.
