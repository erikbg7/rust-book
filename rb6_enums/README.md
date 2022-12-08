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

