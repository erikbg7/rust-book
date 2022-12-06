# Rust Book: Common Concepts

## Constants
- Written in uppercase with underscores between words.
- Can be declared in any scope, including the global scope.
- Type must be explicitly annotated.

----

## Shadowing
Declare a new variable with the same name as a previous variable: the first variable is shadowed by the second (the second one will be seen by the compiler instead).

The second variable overshadows the first, taking any uses of the variable name to itself until either it itself is shadowed or the scope ends.

```rust
fn main() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}"); // 12 (only inside scope)
    }
    println!("The value of x is: {x}"); // 6
}
```

### Differences with `mut`: 
- We have to use `let` to reassign a variable.
- By using `let`, we can perform a few transformations on a value but have the variable be immutable after those transformations have been completed.
- Shadowing is equivalent to creating a new variable.
- We can change the type of the value but reuse the same name.

```rust
let spaces = "   "; // spaces is a string
let spaces = spaces.len(); // spaces in now a number
```

It avoids having to come up with 2 different names (e.g. spaces_str and spaces_num).
But we are not allowed to mutate a variable's type with `mut`:

```rust
let mut spaces = "   "; // spaces is a string
spaces = spaces.len(); // compiler error[E0308]: mismatched types
```
 ---- 

## Data Types

Rust is a **statically typed language** (it must know the types of all variables at compile time) 

### Scalar Types

4 primary types: integers, floating-point numbers, booleans, and characters.

- Integers: `u8`, `i8`, `u16`, `i16`, `u32`, `i32`, `u64`, `i64`, `u128`, `i128` (Number of bits they take in memory. Additionally, we have `isize` and `usize` types depend on the architecture of the computer your program is running on: 64 bits if you’re on a 64-bit architecture and 32 bits if you’re on a 32-bit architecture).
- Floating-point numbers: `f32`, `f64` (All floating-point types are signed).
- Booleans: `bool` (1 byte in size)
- Characters: `char` (most primitive alphabetic type; has to be specified using single quotes; 4 bytes in size; emojis included)

isize and usize types depend on the architecture of the computer your program is running on, which is denoted in the table as “arch”: 64 bits if you’re on a 64-bit architecture and 32 bits if you’re on a 32-bit architecture

### Compound Types

- Touples:
    - Group together different types in a compound type.
    - Tuples have a fixed length: once declared, they cannot grow or shrink in size.

    ```rust
        let tup: (i32, f64, u8) = (500, 6.4, 1);

        let (x, y, z) = tup; // deestructuring

        let five_hundred = tup.0; // accessing values
    ```

- Arrays:
    - Every element must have the same type.
    - Useful for:
        - Data allocated on the stack (not heap).
        - Ensure a fixed number of elements.
    - Only `Vectors` can grow or shrink in size.

    ```rust
        
        let a: [i32; 5] = [1, 2, 3, 4, 5]; // [type, length]

        let b = [3; 5]; // [value, length] = [3, 3, 3, 3, 3]

        let first = a[0]; // accessing values
    ```
    
    If we try to access an element out of bounds, we will get a runtime error. We won't be able to point to an invalid value, Rust will panic and finish the program instead.
    

    ```rust
        let index = 10;
        let arr = [1, 2, 3, 4, 5];
        let element = arr[index]; // runtime error: index out of bounds: the len is 5 but the index is 10
    ```
















