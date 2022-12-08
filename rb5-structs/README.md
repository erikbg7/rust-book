# Rust Book: Structs

Define a struct:
- use the `struct` keyword
- name it in PascalCase
- define the `fields`

```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
```

Crete and instance of the struct:

```rust
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
```

To get values from a struct we can use dot notation:
  
  ```rust
    let email = user1.email;
  ```

If the instance is mutable we can use dot notation to change the value as well:

`The entire struct must be mutable, we cannot specify a single field as mutable.`

```rust
    user1.email = String::from("anothername@example.com");
```

We can use the struct update syntax to create a new instance from an existing one:

```rust
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1 // note that it comes after the other fields
    };
```

- it uses `=` because it moves the data
- user1 is no longer valid after user2 creation
- if all the fields had implemented the Copy trait, we would still be able to use user1, because the values would be copied instead.
- in this case the username is a String, so user1 becomes invalid.

User is still valid if we do the following, because the other fields implement Copy:

```rust
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("username2"),
        ..user1
    };
```



## Tuple Stucts Without Named Fields

- Do not have named fields
- We can use destructure and use dot notation to get values

```rust
  struct Color(i32, i32,i32)
  let black = Color(0,0,0)
```


## Struct Without Fields

- Useful main to implement traits on types that do not have any fields

```rust
  struct AlwayEqual;
  let something = AlwayEqual;
```

## Ownership of Struct Data

We want each instance to own all of its data, so the data is valid for as long as the entire struct is valid.

- It is possible for struct to store references to data owned by something else using `lifetimes`.
- For the moment, it is better to use owned types such as String.


## Displaying Structs

- Values from a struct cannot simply be displayed using `println!` macro
- We can use `{:?}` to display the struct
- But first we must add the attribute to it `#[derive(Debug)]`

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

...

    println!("rect1 is {:?}", rect1);
```

- We can also use `dbg!` macro to display any value

```rust
    let rect1 = Rectangle {
        width: dbg!(30 * scale), // will print 30*scale=60 and return the same value
        height: 50,
    };

    dbg!(&rect1); // we dont want dbg to take ownership, so we send a reference
```

----

## Methods in Structs

- We use the implementation block (`impl`)
- We use the `&self` parameter to access the struct fields

We use the reference `&` because we do not want the Self instance to be consumed/moved, we wanna borrow it.

Another way to see it is: we do not want to take ownership of the instance, we want to borrow it.

**Automatic referencing and dereferencing**

The following are the same, Rust handles borrowing in methods.
```rust
p1.distance(&p2);
(&p1).distance(&p2);
```


### Associated Functions

- do not have `&self` as a first parameter
- thos functions are NOT methods
- are called using `::` syntax: `Rectangle::square(3);`
- often used to return new instances of the struct


The following function returns a new instance of the Rectangle where the width and height are the same:

```rust
impl Rectangle {
    fn square(size: u32) -> Self { // type of the impl Struct
        Self {
            width: size,
            height: size,
        }
    }
}
```
