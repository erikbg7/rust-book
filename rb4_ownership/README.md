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



https://medium.com/@ajeet214/image-type-conversion-jpg-png-jpg-webp-png-webp-with-python-7d5df09394c9