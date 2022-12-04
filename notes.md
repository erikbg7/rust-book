# Rust Book

## Installation
Check Rust version
```
rustc --version
```

Update Rust version
```
rustup update
```

Open Rust documentation
```
rustup doc
```


## Getting Started

### 1. Two things to consider:

- We need a `main` function to compile and run isolated file

- Rust style is to indent with four spaces, not a tab.

- `println!` calls a Rust macro. If it had called a function instead, it would be entered as println (without the !). Using a ! means that you’re calling a macro instead of a normal function, and that macros don’t always follow the same rules as functions.


### 2. How to run the code:

We need to compile the code first, then run it:
```
rustc hello_world.rs
./hello_world
```
After compiling successfully, Rust outputs a binary executable.

## Cargo
Cargo is Rust’s build system and package manager. Most Rustaceans use this tool to manage their Rust projects because Cargo handles a lot of tasks for you, such as building your code, downloading the libraries your code depends on, and building those libraries (dependencies).

Check Cargo version
```
cargo --version
```

Init a new project called `hello_cargo`

```
cargo new hello_cargo
cd hello_cargo
```

Init a Cargo project in an existing directory
```
cargo init
```

It will create a `Cargo.toml` file.
The last line in the new file, [dependencies], is the start of a section for you to list any of your project’s dependencies.
In Rust, packages of code are referred to as crates.


You cannot mutate something if you do nnot own it.