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


### Data Types

Rust is a *statically typed* language, which means that it must know the types of all variables at compile time.

```rust
let guess: u32 = 23;
```
<ins>**Scalar types**</ins>

There are 4 primary scalar types: 

1. integers
2. floating-point numbers
3. Booleans
4. characters




* Integer Types in Rust:

| Length  | Signed  | Unsigned |
|---------|---------|----------|
| 8-bit   | i8      | u8       |
| 16-bit  | i16     | u16      |
| 32-bit  | i32     | u32      |
| 64-bit  | i64     | u64      |
| 128-bit | i128    | u128     |
| arch    | isize   | usize    |


Quick reminder: 
> signed : integer can be negative

> unsigned: integer cannot be negative


* Float Types in Rust:

| Length  | Signed  | Precision       |
|---------|---------|-----------------|
| 32-bit  | f32     | Single-precision|
| 64-bit  | f64     | Double-precision|


* Boolean:
```rust
let mut state : bool = true;
state = false;
```

* Character:
```rust
let c = 'Z';
```
They take four bytes in size and represents a **Unicode Scalar Value**, meaning it can represent a lot more than just ASCII. I won't go more in depth in this file.

<ins>**Numeric Operations**</ins>

They are like in most languages


<ins>**Compound Types**</ins>

These types can group multiple values into one type. Rust has two primitive compound types:
1. Tuples
2. Arrays


* The Tuple Type:

Tuple have a fixed length, and therefore cannot grow not shrink in size.

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);
```
or we can even do:

```rust
let tup = (500, 6.4, 1);

let (x,y,z) = tup;
```

to assign multiple values in one line.

This is called *destructuring* because it breaks the single tuple into three parts.


To access elements from a tuple we can use `.` followed by the desired index.

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);

let first = tup.0;
```


We call a tuple without any values, **a unit**.

* The Array Type:

Same as for tuples in terms of fixed size, but every elements of an array must have the same type. (Vectors are arrays that are allowed to change size, we will discuss them later)


```rust
let arr = [1,2,3,4,5];
```
or when you want to specify the type and size:
```rust
let a: [i32, 5] = [1,2,3,4,5];
```


To initialise an array that countains the same value repeated we can just do
```rust
let cheeses = ["cheese"; 5];

println!("PrintCheese: {}", cheeses[0]);
```

As shown above, to access arrays, we can simply add `[i]` with i the index of the desired value. 

### Functions

To call a function, like most language we can do

```rust
fn main(){
    another_function();
}

fn another_function(){
    println!("I was called");
}
```

Important to note that **rust doesn't care where we define our functions**, only that theyre defined somewhere in a scope that can be seen by the caller.

* **Parameters**:
```rust
fn another_function(x: i32){
    println!("The value of x is: {x}");
}
```

* **Statements and Expressions**:

> `Statements` are instructions that perform some action and **do not return a value**.

> `Expressions` evaluate to a resultant value. They do not end with **semicolons**

This means we can do stuff like this
```rust
fn main(){
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}");
}
```
which will result in
```bash
The value of y is 4
```
because the block evaluates to 4. 

`let x = 3;` is a statement, so it will not return a value.

`x + 1` is an expression which will return 4 after being evaluated. 

All of that to lead into this next part :)

* **Functions with Return Values**:

In Rust, the return value of a function is the value of the final expression in the block of the body. (But we can also return a value early by usingf the `return` keyword and specifying a value)

```rust
fn five() -> i32{ // We have to specify the return type
    5
}
fn main(){
    let x = five();
    println!("The value of x is: {x}");
} 
```
Or
```rust
fn plus_one(x: i32) -> i32{
    x + 1 // If we add a semicolon here, we will get an error
}
fn main(){
    let x = plus_one(5);
    println!("The value of x is: {x}");
} 
```


### Control Flow

* **if Expressions**:

They work like in most languages.
```rust
fn main(){
    let number = 6;

    if number % 4 == 0{
        // Do something
    } else if number % 3 == 0{
        // Do something else
    }else{
        // Do that
    }
}
```

We must explicitly provide `if` with a **Boolean** as it's **condition**.

* **Using if in a let Statement**:

> `if` is an expression, we can use it on the right side of a `let` statement to assign the outcome to a variable.
>> The values that have the potential to be results from each arm must be the same type.

```rust
fn main(){
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}
```

### Repetition with Loops

* **Repeating Code with loop**:

The `loop` keyword tells Rust to execute a block of code over and over **until** we tell it to stop. (For exemple, in the terminal with `ctrl-c`)

```rust
fn main(){
    loop{
        println!("again!");
    }
}
```

We can also exit the loop in code by using the `break` keyword. 

And use the `continue` keyword to skip over remaining code in the block during the current iteration to skip to the next one.



* **Returning Values from Loops**:

It can be useful to know how to return values from loops. To, for example, retry an operation that might fail (Checking wether a thread has completed its job).

To do this, simply add the returned value after the `break` keyword.
```rust
fn main(){
    let mut counter = 0;        // Do not forget the mut keyword
    
    let result = loop{
        counter += 1;
        
        if counter == 10{
            break counter * 2;
        }
    }
    println!("The result is {result}");     // 20
}
```
* **Loop Labels to Disambiguate Between Multiple Loops**:
```rust
fn main(){
    let mut count = 0;
    'counting_up: loop{
        let mut remaining = 10;

        loop{
            println!("remaining = {remaining}");
            if remaining == 9{
                break;      // Will break the inner loop
            }
            if count == 2{
                break 'counting_up;     // Breaks the 'count_up loop
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
}
```
Will result in:
```bash
remaining = 10
remaining = 9
remaining = 10
remaining = 9
remaining = 10
End count = 2
```


* **Sub-Chapter**:
```rust
```