# Rust Book: Enums

- Define a value by enumerating its possible variants
- Use PascalCase 

```rust
enum IpAddrKind {
        V4,
        V6,
    }
```

We can create instances of the enums:

```rust
let ipv4 = IPAddrKind::V4;
let ipv6 = IPAddrKind::V6;
```

----

We could crate a `struct` to represent an IP address, with a field to define to IP kind:

```rust
    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }
```

However it is not the best choice in this specific case.
If we wanna implement methods based on the `IpAddrKind` it will become cumbersome.

We could bind the enum with a value that will represent the address:

```rust
    enum IpAddr {
        V4(String),
        V6(String),
    }
```

But it does not support complex definition either.

----

**How to do it the right way**
- Each specific implementation will have a `struct`
- Structs will be defined as values of an enum

```rust
struct Ipv4Addr {}

struct Ipv6Addr {}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
```

We can put any type of data inside an enum.
The types do not need to even have a relation between them.

```rust
enum Message {
    Quit,  // no data associated
    Move { x: i32, y: i32 }, // named fields like an struct
    Write(String), // sinlge string
    ChangeColor(i32, i32, i32),  // 3 integers
}
```

## Option Enum

- scenarios where `a value could be something or nothing`
- Rust does not have `null`
- Rust represents the value of null through the `Option` enum

```rust
enum Option<T> {  // T is a generic type parameter
    None, // no value
    Some(T), // some value
}
```

```rust
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y; // BIG FAIL
```

We cannot directly add a `i8` to an `Option<i8>`.
If we wanna use the Option value we have to handle explicitly the `None` case.

```rust
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y.unwrap(); // unwrap() returns the value inside the Some variant
```

## `Match` Control Flow

- Compares a value to a series of patterns.
- Patterns can be literals, variable names, wildcards, and many other things.
- Arms patter MUST cover all the possibilities (matches are `exhaustive`)
- We can use `_` as a catch-all pattern for the match
- We can use the unit value `_ => ()` to avoid running any code for the remaining cases

```rust
#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
```

Another example where we do not know if option is valid:

```rust
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }
```

----

## `if let` Control Flow

- handle values that match one pattern while ignoring the rest

```rust
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
```

We can use an `else` block to handle non-matching cases.

```rust
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
```

