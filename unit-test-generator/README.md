# Unit-test-generator 
Tool that generates unit test for different languages such as
Rust, Python and C++ using AI models.

## Build
Use the following command line to build the tool:

*in debug*
```
cargo build 

```

*in release*
```
cargo build --release

```

*in release for target CPU*
```
RUSTFLAGS="-C target-cpu=native" cargo build --release

```
## Generate the unit tests 
Use the following command line to generate the file that contains the unit tests:

Generate tests for Rust
```
cargo run -- <file.rs>

```

Generate tests for C++ 
```
cargo run -- <file.cxx>

```

Generate tests for Python 
```
cargo run -- <file.py>

```

## To run tests
todo

