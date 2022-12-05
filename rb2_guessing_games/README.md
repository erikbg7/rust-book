# Rust Book: Guessing Game


## Concepts

- In Rust, variables are immutable by default, meaning once we give the variable a value, the value won't change.
```
let apples = 5; // immutable
let mut bananas = 5; // mutable
```

- The :: syntax in the ::new line indicates that new is an associated function of the String type.
```
let mut s = String::new();
```

- The & indicates that this argument is a reference, which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times.

- Result's variants are Ok and Err. The Ok variant indicates the operation was successful, and inside Ok is the successfully generated value. The Err variant means the operation failed, and Err contains information about how or why the operation failed.

- Values of the Result type, like values of any type, have methods defined on them. An instance of Result has an expect method that you can call. If this instance of Result is an Err value, expect will cause the program to crash and display the message that you passed as an argument to expect.

- Will build documentation provided by all of your dependencies locally and open it in your browser
```
cargo doc --open
```

- The `parse` method on strings converts a string to another type. Here, we use it to convert from a string to a number. We **need to tell Rust the exact number type we want by using let guess: u32**.

- We can use `match` to handle the different results (Ok, Err) because `Result` is an **enum**