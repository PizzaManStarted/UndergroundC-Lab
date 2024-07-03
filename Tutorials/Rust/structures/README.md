# Structures In Rust

Like in **C**, we can define structures to help storing multiple related values.

## Defining and Instantiating Structs

### Definition

```rust
struct User {
    active : bool,
    username : String,
    email : String,
    sign_in_count : u64
}
```

### Instantiation

There are many ways to instantiate a struct:

```rust
let user1 = User {
    active: true,
    username: String::from("someusername123"),
    email: String::from("someone@example.com"),
    sign_in_count: 1,
};
```

And if we have variables with the same name as the fields of the struct, we can simplify the call :

```rust
let username = String::from("someusername123");
let email    = String::from("someone@example.com");

let user1 = User {
    active: true,
    username,
    email,
    sign_in_count: 1,
};
```

> [!NOTE]
> This can be useful when creating constructors functions.

### Struct update Syntax

What if we need to create a new struct using some values from a previously created one ?
One simple way to do it, would be :

```rust
let user2 = User {
    active: user1.active,
    username: user1.username,
    email: String::from("another@example.com"),
    sign_in_count: user1.sign_in_count,
};
```

But **Rust** has another way :

```rust
let user2 = User {
    email: String::from("another@example.com"),
    ..user1
};
```

`..user1` must come last in order to fill any remaining fields.

> [!WARNING]
> By doing this, we moved the ownership of the string in `username` from the *user1* to the newly created *user2*, meaning *user1* is no longer a valid reference after this action !



### Tuple Structs

Tuple structs are structs but where you only define the type of value stored but not the name of the attributes. Meaning you only give a name to the struct. This can help when wanting to differentiate tuples.

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin :Point = Point(0, 0, 0);
    
    println!("Access data with a . and the index: {}", black.0);
    // But doing 
    // let origin : Point = Color(0,0,0);
    // is forbidden even if they have the same fields
}
```

### Unit-Like Structs Without Any Fields

```rust
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
}
```

They can be usefull when we need to store a state or something similar.

> [!IMPORTANT]
> We used `String` instead of `&str` to define the `User` struct, this is in order to not get any compilor error because we have not learned about **LifeLines** yet.



## Printing a Struct

Since our structs do not implement `Display`, we cannot simply do :

```rust
let rect1 = Rectangle {
    width: 30,
    height: 50
};
println!("rect1 is {rect1}");
```

to print our struct. But we can specify `#[derive(Debug)]` in order to print the struct.

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {rect1:?}"); // or {rect1:#?} to get a pretty print with tabulations
}
```


> [!IMPORTANT]
> In order to get more informations about derivations we can use the `dbg!(..)` function that will show the given operation and return the computed value.

```rust

dbg!(3 +1  - 1 /2 ); // Will print : 3 + 1 - 1 / 2 = 4

let scale = 2;
let rect1 = Rectangle {
    width: dbg!(30 * scale),    //  Will print :  30 * scale = 60
    height: 50,
};
dbg!(&rect1); // Will print : &rect1 = Rectangle {
//                                          width: 60,
//                                          height: 50,
//                                     }
```

## Method Syntax

They are defined in the context of a struct and must start with `&self` as their first parameter, reprensenting the instance of the struct calling the method.

### Method Definition

Start by creating an `impl` (*implementation*) block 

> [!NOTE]
> The `&self` is actually short for `self: &Self`. Within an impl block, the type Self is an alias for the type that the impl block is for.

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
```

We can also note that methods can take ownership of `self` (or borrow like we did here). Here we can only read values from the given struct but if we wanted to be able to modify it, we would simply write : `fn area(&mut self) ...`

We can use just `self` if we want to stop the user from using the variable again after the method is called.

> [!IMPORTANT]
> We call the method by doing : `.method_name(..)`. The parenthesis shows to **Rust** that this is a method and not a field. So we can have a method and a field of the same name without any issues. This is a way to create **getters**.

We can also add more parameters :

```rust
fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
}
```

### Associated functions

All functions defined in a `impl` block are called **associated functions** but we can specify functions without the first `self` parameter (meaning they aren't methods). This can be useful when creating **constructors** for structs.

And we use the `::` syntax in order to call them, exemple : `let sq = Rectangle::square(3);`

```rust
impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
```

### Seperated impl blocks

Nothing is stopping us from having multiple seprated `impl` blocks for a specific struct. They aren't useful right now, but will be when discussing generic types and traits.
