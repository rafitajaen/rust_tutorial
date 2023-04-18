# Rust Tutorial

## How to install Rust on Windows

1. Download and install: https://www.rust-lang.org/tools/install
2. Download and install https://visualstudio.microsoft.com/es/vs/
3. Install C++ Build Tools para escritorio
   3.1. MSCV v.142+
   3.2. Windows 10 SDK
   3.3. C++ CMake tools for Windows

### Rust Version

```shell
$ rustc --version
```

### How to manage version of Rust

```shell
$ rustup update
```

### How to uninstall Rust

```shell
$ rustup self uninstall
```

## Cargo

Cargo is Rust's build system and package manager

```shell
$ cargo --version #Get Cargo's version

$ cargo new <project-name> #Create a new project
$ cargo new --vcs=git #Create a new project with a different version control system
$ cargo new --help #Help for a new projects command

$ cargo build #It just compiles and creates executable [./target/debug]
$ cargo run #It compiles and then, run the resulting executable [./target/debug]
$ cargo check #It just checks if the code still compiles. Faster than build.

$ cargo build --release #It compiles code with optimizations. Slow. [target/release]

$ cargo clippy #For warnings
```

## Silence Warnings

```rust
#![allow(unused_variables)]
```
