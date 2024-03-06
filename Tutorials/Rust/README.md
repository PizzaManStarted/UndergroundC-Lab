# Learning Rust

## Building a Project with Cargo
Cargo allows us to write more complex program by making easier the process of adding dependencies.


### Creating a Project

```bash
cargo new name_of_project
cd name_of_project
```

This will generate many things such as:
* main.rs file
* Cargo.toml file
* a new Git repository with a git ignore file

To not generate a git repository and we can just add 

```bash
cargo new name_of_project --vcs none
```
To simply Run the project we can just do (while inside the created folder)
```bash
cargo run
```
(Or we can build then execute the resulting file)


We can also check if the code is able to compile or not by doing
```bash
cargo check
```

### Adding dependencies

To add dependencies to a project simple change the *Cargo.toml* file
```toml
[dependencies]
rand = "0.8.5"
```
For exemple here, we added the rand **crate** with the semantic version specifier 0.8.5, this last part is equivalent to **^0.8.5**, which means any version that is at least 0.8.5 but below 0.9.0. 
___
## Common Programming Concepts

### Mut and Non Mutable variables

Let's take this program
```rust
fn main() {
    let x = 5;
    println!("Value of x is: {x}");
    x = 6;
    println!("Value of x is: {x}");
}
```
This will produce an error message when ran regarding an immutability error.

The reason being that we tried to assign twice an **immutable** variable. 

> Variables are **immutable** by **default**

We can make them mutable by adding `mut`

```rust
fn main() {
    let mut x = 5;
    println!("Value of x is: {x}");
    x = 6;
    println!("Value of x is: {x}");
}
```
### Constants

> Constants are values that are bound to a name and are not allowed to change.
>> Constants may be set only to a *constant* expression, not the result of a value that could only be computed at runtime.

```rust
const CONST_NAME: type = value;
```



For example:

```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```
Will be evaluated during the compile-time to 10800.


### Shadowing
This is kind of new from other languages.

We can declare a new variable with the **same** name as the previous variable. This process is called **shadowing**.

```rust
let x = 5;
println!("Value of x is: {x}");
let x = 6;
println!("Value of x is: {x}");
```
Or even
```rust
let x = 5;
{
    let x = 6;
    println!("Value of x is: {x}");
}
println!("Value of x is: {x}");
```
Will output
```bash
Value of x is: 6
Value of x is: 5
```

This can be useful when trying to change an unmutable variable while still making sure it stays unmutable after changes are made.

Or even to change the **type** of a variable


This will produce an error
```rust
let mut spaces = "  ";
spaces = spaces.len();
println!("Value of x is: {spaces}");
```

meanwhile
```rust
let spaces = "  ";
let spaces = spaces.len();
println!("Value of x is: {spaces}");
```
will not raise any issues.


## Data Types



Integer Types in Rust:

| Length  | Signed  | Unsigned |
|---------|---------|----------|
| 8-bit   | i8      | u8       |
| 16-bit  | i16     | u16      |
| 32-bit  | i32     | u32      |
| 64-bit  | i64     | u64      |
| 128-bit | i128    | u128     |
| arch    | isize   | usize    |




