# Exercise solutions for "learn Rust by solving 100 exercises"

## Getting started

Go to [rust-exercises.com](https://rust-exercises.com) and follow the instructions there
to get started with the course.

## How to run all quality assurance checks

To run all quality assurance checks, run the following command:

```bash

cargo make qa
```

This will run all quality assurance checks, including: `cargo fmt`, `cargo clippy`, `cargo test`.
This assumes you have `cargo-make` installed. If you don't have it installed, you can install it by running:

```bash

cargo install --force cargo-make
```
