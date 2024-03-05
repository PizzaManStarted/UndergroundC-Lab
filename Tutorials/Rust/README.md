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
cargo new name_of_project --vsc none
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
For exemple here, we added the rand **crate** with the semantic version specifier 0.8.5, this last part is equivalent to ^0.8.5, which means any version that is at least 0.8.5 but below 0.9.0. 

___
## Hello World
```rust
fn main(){
  println!("Hello World!");
}
```

Here **println!** is not a function but **Rust marcro**.
