# primes
A Rust based lib on finding prime numbers (small).

# Building

* Install the latest version of Rust, if you don't alrady have it from https://www.rust-lang.org/tools/install
* Execute `cargo build --release`

# Tests

To execute tests simply run `cargo test` which enables test configuration to run unit tests and doc tests.

# Running

There are two modes to run the primes binary (which can also be used as a library crate):
* To find whether a number is prime - provide a number as a parameter
* To find greatest prime number in a range - provide `<start>` and `<end>` as parameter.

## Examples

The examples assume you have exported the resulting binary from the crate to your path:

```bash
$ export PATH=$PATH:$(pwd)
```

### Find whether a number is prime

```bash
# To check whether a number is prime
$ primes 199
```

### Find greatest prime in a range

```bash
# To find greatest prime number in a range
$ primes 100 200
```