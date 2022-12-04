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

