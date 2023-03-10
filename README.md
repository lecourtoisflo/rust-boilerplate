# rust-boilerplate
small boilerplate project for RUST
 
Contains:
- a package with a library to manage a thread-based periodic task
- a parent package with a small executable using it

## Requirements
RUST installation (see https://doc.rust-lang.org/book/ch01-01-installation.html)

## Compilation
 To compile the task library in task directory:
 - `cargo build` to build the library
 - `cargo test` to run the unit tests
 - `cargo run --example basic` to run the defined example

 To compile the boilerplate executable in main directory:
 - `cargo build`to build the executable: this will build the dependency task library
 - `cargo run` to run the executable

 ## Developement
 Under VSCode, the following extensions are recommended:
 - [rust-analyser](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) for rust parser
 - [Better TOML](https://marketplace.visualstudio.com/items?itemName=bungcip.better-toml) for cargo manifest
 - [Cargo](https://marketplace.visualstudio.com/items?itemName=panicbit.cargo) for cargo integration
