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

Important to note that **rust doesn't care where we define our functions**, only that they are defined somewhere in a scope that can be seen by the caller.

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

All of that to lead into this next part

* **Functions with Return Values**:

In Rust, the return value of a function is the value of the final **expression** in the block of the body. (But we can also return a value early by using the `return` keyword and specifying a value)

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

We can specify a loop *label* on a loop that we can then use with `break` or `continue` to specify to which loop the keyword should apply. **Loop Label must begin with a single quote**.

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

* **Conditional Loops with while**:

Like in all languages

```rust
fn main(){
    let mut number = 3;

    while number != 0{
        println!("{number}!");

        number -= 1;
    }
    println!("LIFTOFF!!!");
}
```

* **Looping Through a Collection with for**:

We can loop through a collection such as an array with many ways such as with a `while` loop or even using a `for` loop.

```rust
fn main(){
    let a = [10,20,30,40,50];

    for element in a{
        println!("The value is: {element}");
    }
}
```

We can also loop through a range, like in *python*.
```rust
fn main(){
    for element in (1..4).rev(){
        println!("The value is: {element}");
    }
}
```

## Memory Fun

In Rust, memory works a bit differently than for different languages.

### String type vs litterals

Let's take the easiest way to understand this using strings.

```rust
let mut s = "Hello";
```

This string is litteral and therefore, we cannot add/append anything to this string during runtime. It is not mutable and are *hardcoded* values.

To have a string with a size different we can use the `String type`:

```rust
let mut s = String::from("Hello ");
s.push_str("world"); // and to append something we can use `.push_str(..)`
```

### Allocation and Freeing

In other language like **C**, we have to ask for memory then free it by hand using `malloc(..)` and `free(..)`. And some languages like **Java** use a Garbage collector that allows user to not worry about memory.

Rust is quite different. When we previously used `String::from("..")`, it allocated the requested memory during runtime.  `s` is now a pointer and the owner of this string.

But how do you return this memory to the allocator when we are done with our string ?

> [!IMPORTANT]  
> The memory is automatically returned once the variable that owns it goes out of scope.

This is the most important rule to remember and understand.

What do you think will happen if we compile this ?

```rust
let s1 = String::from("I sure hope no one will steal my string");
let s2 = s1;
println!("s1: {s1}");
```

:boom: We will get an error, but why ?

Because the reference `s1` became invalidated after the second line. We call this a **move**.

> [!NOTE]  
> A move can be considered as a shallow copy to a second variable that makes the first **invalid**.

But still why is this the case in rust ? Well since the memory of a variable is freed after goes out of scope, if we had two (or more) pointers pointing at the same variable, it would try to free the same thing multiple times. This can cause many issues in other languages like in *C* with the *segmentation faults*. Rust allows us to not do this mistake.

#### Move & Copy

If you have been following something might alarm you, what if we do this :

```rust
let x = 5;
let y = x;
println!("x = {}, y = {}", x, y);
```

Shouldn't it also throw an error ? Or this ?

```rust
let s = "Hello World";
let p = s;
println!("{p} {s}");
```

This code is valid and will not cause any problem because primitive values are **(deeply) copied** automatically.

This is because intergers implement the `Copy trait`. (We will discuss what are traits later). Some of the other type that implement it are:

* All the integer types, such as u32.
* The Boolean type, bool, with values true and false.
* All the floating-point types, such as f64.
* The character type, char.
* Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.

We also just saw how to do a *shallow copy* of a string with a **move**, but what if we still desire to make a *deep copy* of the string ? We can do that using `clone()` which is a common method for many types.

```rust
let s1 = String::from("I sure hope no one copies my string");
let s2 = s1.clone();
println!("s1 : {s1}, s2 : {s2}"); // This is valid
```

### Ownership and functions

*Assignments* are not the only way to transfer the ownership of a value from a variable to another. This can also happen when passing a variable as an argument to a function.

```rust
fn main() {
    let p = String::from("hello");  // p comes into scope

    takes_ownership(p);             // p's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

} // Here, x goes out of scope, then p. But because p's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
```

* **Return-Values**:

We can also pass ownership by returning a value

```rust
fn main() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}
```

> [!IMPORTANT]  
> The main idea behind ownership is to keep the number of variables that can **modify/wr** a value to the strict minimum in order to prevent common low level errors such as `data races` and `dangling pointers`.

But as you can guess, keeping track of every returned value or passed parameter and being forced to create a new variable for each is not the easiest thing to do and can get pretty tedious. Thankfully **Rust** has some other features in order to facilitate this.

#### References and Borrowing

> [!IMPORTANT]
> A reference is like a pointer in that it’s an address we can follow to access the data stored at that address. That data is owned by some other variable. Unlike a pointer, a reference is guaranteed to point to a valid value of a particular type for the life of that reference.

We use `&` to declare references.

References do not have ownership of values, therefore when reaching the end of the scope, it will not free the pointed data.

For exemple:

```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1); // &s1: Get a reference to the value of s1

    println!("The length of '{s1}' is {len}."); // s1 is still usable
}

fn calculate_length(s: &String) -> usize {
    s.len() // s is a reference
}
```

You cannot **modify** a borrowed value ! So it is `Read Only`. Just as variables are immutable by default, so are references. We’re not allowed to modify something we have a reference to.

* **Mutable Reference**:

But we can use `&mut`:

```rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) { // mutable reference
    some_string.push_str(", world");
}
```

> [!IMPORTANT]
> Mutable references have one big restriction: if you have a mutable reference to a value, you can have no other references to that value

This is in order to prevent ***Data Races***

Keep in mind this rule is not restricted to when someone creates a reference but also when assigning `mut` variables.

> [!IMPORTANT]
> If you have a reference to a mutable value, you can have multiple references to this value (i.e. in Read only) but cannot **also** have a **mutable** reference poiting to the value. `We cannot have a mutable reference while we have an immutable one to the same value.`

```rust
let mut s = String::from("hello");

let r1 = &s; // no problem
let r2 = &s; // no problem
let r3 = &mut s; // BIG PROBLEM
// [error] cannot borrow `s` as mutable because it is also borrowed as immutable
println!("{}, {}, and {}", r1, r2, r3);
```

With this, users of an immutable ressource can trust that this ressource will not change while they are reading it.

> [!WARNING]
> The scope of reference for immutable variables is *different* from the one we are used to. Meaning the compiler will not throw an error if you get an mutable reference even if immutable reference currently exist if those are not *used* anymore.

So this following code block will not cause any problem :

```rust
let mut s = String::from("hello");

let r1 = &s; // no problem
let r2 = &s; // no problem
println!("{r1} and {r2}");
// variables r1 and r2 will not be used after this point

let r3 = &mut s; // no problem
println!("{r3}");
```

> [!NOTE]
> Thanks to these rules, it is not possible to have any **dangling** pointers


... more to come

* **Sub-Chapter**:
