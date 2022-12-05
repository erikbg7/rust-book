# Rust Book: Common Concepts

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

