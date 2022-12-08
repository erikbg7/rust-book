# Rust Book: Ownership
Enables memory safe guarantees without a garbage collector.

There are 3 main ways to manage memory:
- Garbage Collection: regulary looks for unused memory and frees it.
- Explicit Memory Management: programmer manually allocates and frees memory.
- Ownership: Memory is managed through a system of ownership with a set of rules that the compiler checks at compile time.

Rust uses this last approach.

----

## The stack and the heap
Both are part of the memory available at runtime, but are structured in different ways.

### Stack
 - LIFO (Last In First Out)
 - All data must have a known, fixed size.
 
### Heap
- The memory allocator searches an empty spot in the heap that is big enough to fit the data.
- When we allocate we get a pointer (the address of the location)

### Considerations
- Data with unknown or variable size must be stored on the heap.
- Stack is faster because it does not have to find a new place to store the data, will be always on top.
- Similarly, accessing data on the Heap is slower because we have to follow the pointer to get there.
- e.g. when the code calls a function the values passes to it are pushed to the stack, when the function finishes those values are popped off the stack.

### Summary
- Ownership addresses the issue of managing duplicated, unused, etc. data on the Heap.
- Main purpose of the ownership is to manage heap data.

----

## Ownership Rules
- Each value in Rust has an owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

This kind of string can be mutated, but literals cannot.
The only difference is hoy the types deal with memory.

```rust
  let mut s = String::from("hello");
  s.push_str(", world!"); // push_str() appends a literal to a String
  println!("{}", s);
```

----

## Memory and Allocation: Move

- string literals are fast and efficient, the contents are known at compile time. So we can harcocode it in the final executable.
- using the `String` type we need to support a mutable, growable piece of text.
- using the `String` we have to allocate an amount of memory on the heap.
- Memory must be requested at run time.
- Memory will be "droped" when the variable goes out of scope.
 
 ```rust
  {
    let s = String::from("hello"); // s is valid from this point forward
    // do stuff with s
  } //scope is now over, and s is no longer valid
 ```

*EXAMPLE 1*
```rust
let x = 5;
let y = x;
```
- Bind value 5 to x, make a copy of the value in x and bind it to y.
- We can do this because the values are known, fixed.
- Both values are pushed onto the stack.


*EXAMPLE 2*
```rust
let s1 = String::from("hello");
let s2 = s1;
```
**STACK: s1 pointer**

| name     | value  |
|----------|--------|
| ptr      | @heap0 |
| len      |   5    |
| capacity |   5    |

**HEAP: s1 content**

| index | value  |
|-------|--------|
|   0   |   h    |
|   1   |   e    |
|   2   |   e    |
|   3   |   l    |
|   4   |   l    |
|   5   |   o    |

- len: memory in bytes used by the content
- capacity: total memory in bytes allocated for the content

When `s1` is assigned to `s2`, the pointer, the length and the capacity are copied and pushed to the stack, but the heap allocation remains the same.

Copying everything again the heap could be expensive and slow, so Rust does not do it.

The problem now is that `s1`and `s2`could try to free the same memory allocation twice, which would cause something known as a "double free error".

To ensure memory safety, Rust considers `s1` to no longer be valid after `let s2 = s1`. Using `s1` will cause *error[E0382]: borrow of moved value: s1*.

In terms of shallow copy and deep copy, there are 2 things to consider:
- Rust will never create "deep" copies of the data, so `any automatic copying can be assumed to be inexpensive in terms of performance`.
- Instead of saying that s2 is a shallow copy of s2, we say that `s1 was moved to s2`. Because s1 is no longer valid after the assignment.

----

## Memory and Allocation: Copy
We can use `clone` to make a deep copy.

```rust
let s1 = String::from("hello");
let s2 = s1.clone();
```
- clone is expensive
- the heap data is copied (duplicated)

We don't need to use `clone` in the following example:

```rust
let x = 5;
let y = x;
```
- values are entirely stored on the stack
- values have known size at compile time
- no difference between "shallow" and "deep" copy

Rust has a special trait called Copy for types that are stored on the stack (as integers). 
`If a type implements the Copy trait, variables that use it do not move, but rather are trivially copied, making them still valid after assignment to another variable.`

We can implement Copy to our custom types, only if they do not implement the Drop trait and do not require allocation.

Some types that are Copy:
- All integer and floating point types.
- All booleans and characters.
- Tuples only if contain types that are also Copy.


----

## Ownership and Functions

- Passing a variable to a  is similar to assigning a value to a variable.

```rust
fn main() {
    let s = String::from("hello");  // s comes into scope
    takes_ownership(s);             // s's value is moved to the function, no longer valid here

    // using s at this point would throw a compile-time error, value was moved to the function

    let x = 5;                      // x comes into scope
    makes_copy(x);                  // x would move into the function as a copy, we can use it afterward,
}

fn takes_ownership(text: String) { // text comes into scope
    println!("{}", text);
} // text goes out of scope: `drop` is called and memory us freed

fn makes_copy(number: i32) { // number comes into scope
    println!("{}", number);
} // number goes out of scope. Nothing special happens.
```

----

## Return Values and Scope

- Return values can also transfer ownership.


```rust
fn main() {
    let s1 = gives_ownership(); // gives_ownership moves its return value into s1

    let s2 = String::from("hello"); // s2 comes into scope
    let s3 = takes_and_gives_back(s2);  // s2 is moved into function, which return value into s3
} // s1 out of scoped -> droped, s3 out of scope -> droped, s2 moved, nothing happens

fn gives_ownership() -> String { // moves its return value into the function that calls it
    let text = String::from("yours"); // text comes into scope
    text                              // text is returned and moved out to the calling function
}

fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope
    a_string  // a_string is returned and moves out to the calling function
}
```

The ownership of a variable follows the same pattern everytime:
- assigning a valuea to a variable moves it
- heap data will cleaned up by drop when the variable goes out of scope
- to avoid dropping it, the ownership must be moved to another variable

Having to do this everytime is tedious, so Rust has a feature called *references*.

----

## References and Borrowing

- `& is the reference operator`
- Using references we can refer to a value without taking ownership of it.
- A reference is like a pointer to data owned by another varible.
- If we do not own the value, `we cannot mutate it`
- `* is the dereference operator, has the opposite effect`

```rust
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // as it is only a reference, the value is not dropped
```

----

### Mutable References

- To be able to mutate a borrowed value we have to use a `mutable reference`
- Written as `&mut String`

```rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

There is one restriction tho:

*You can only have a SINGLE mutable reference at the same time*

- We can not borrow a value as mutable more than one.
- The benefit is that we  can prevent data races at compile time.

```rust
    let mut s = String::from("hello");
    let r1 = &mut s;
    let r2 = &mut s; // BIG FAIL
    // error[E0499]: cannot borrow `s` as mutable more than once at a time
```

A `data race` could happen if:
- Two or more pointers access the same data at the same time.
- At least one of the pointers is being used to write to the data.
- There’s no mechanism being used to synchronize access to the data.


We can use scopes to have multiple mutable references, but no simultaneous ones.


```rust
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference.

    let r2 = &mut s;
```

There is a similar rule for working with mutable and immutable reference at the same time:

```rust
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

    println!("{}, {}, and {}", r1, r2, r3);
```
- users of an immutable reference do not expect the value to suddenly change from under them.
- if r1 and r2 were used (printed) before r3, no problem would happen (a.k.a NNL or Non-Lexical Lifetimes)

----

### Dangling References

- A reference to a value that is no longer valid is called a `dangling reference`
- Rust prevents dangling references

```rust
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String { // returns a reference to a String
    let s = String::from("hello");
    &s
} // BIG FAIL: reference value is dropped
```

The correct thing is to just return the value, not the reference:

```rust
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> String { // returns a String
    let s = String::from("hello");
    s
} // ownership is moved, nothing is deallocated
```

----

## The Slice Types

- Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection.


```rust
fn main() {
    let mut s = String::from("hello world");
    // first_word takes an immutable reference and returns a usize index
    let word = first_word(&s); // word will be 5

    s.clear(); // empty the String, making it ""

    // PROBLEM: word has value 5, but s has completely changed, so word is now invalid and there is no compile time warning
}
```

In order to have a safer code we can use `slices`. So we can have a reference to a part of the String, and Rust will keep the string immutable until the slice is not used.

```rust

fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s);

    s.clear(); // error!
    println!("the first word is: {}", word);
}


fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```
 `clear` needs a mutable reference, but we already have a mutable one with the slice. We cannot have both at the same time.

 Slices take the form `[starting_index..ending_index]`

 The following slices are equivalent:

  ```rust
  let s = String::from("hello");
  let len = s.len();

  &s[0..2] == &s[..2]
  &s[3..len] == &s[3..]
  &s[0..len] == &s[..]
  ```

- String literals are slices `let s = "hello, world"`
- Arrays also have slices:
```rust
let a = [1, 2, 3, 4, 5];
let slice = &a[1..3];

assert_eq!(slice, &[2, 3]);
```


----

let item = [1,2]
fn do_something(item) // value is "consumed" and gone after using it


let item = [1,2]
fn do_something(&item) // inmutable reference to the value, value is only referenced, not consumed

let mut item = [1,2]

fn do_something(& mut item) // mutable reference, value is can be mutated




# Learn Rust
- 3,4 CLI apps
- 3,4 servers

https://github.com/agarrharr/awesome-cli-apps#image-conversion

# CLI tool project number 1
https://github.com/svg/svgo
- should be able to conver a single image, an array of image files, or a directory of images from JPG, PNG, etc. to WEBP


CLI usage
# Processing single files:
svgo one.svg two.svg -o one.min.svg two.min.svg
# Processing directory of svg files, recursively using `-f`, `--folder` :
svgo -f ./path/to/folder/with/svg/files -o ./path/to/folder/with/svg/output
# Help for advanced usage
svgo --help



