
fn main() {
  // Scope exemple:
  scope_test();
  println!("_________________________");
  memory_allocation();
  println!("_________________________");

  // Primitive are copied
  let s = "test 1";
  let x = s;
  println!("{x} {s}");


  let s1 = String::from("hello");
  let len = calculate_length(&s1);

  println!("The length of '{s1}' is {len}.");

  let mut s = String::from("hello");
  let r3 = &mut s; // BIG PROBLEM
  // cannot borrow `s` as mutable because it is also borrowed as immutable
  println!("{},", r3); // doesn't run

}

// Simple test to demonstrate how scope works
fn scope_test()
{
  // In main scope 
  let s = "hello";
  {
    // This is valid
    let s = "World";
    println!("Scoped s = {s}");
  }
  println!("Exterior s = {s}");

  //  The types covered previously are of a known size, 
  //  can be stored on the stack and popped off the stack 
  //  when their scope is over, and can be quickly and trivially copied to make a new,
  //  independent instance if another part of code needs to use the same value in a different scope.
}

fn memory_allocation()
{
  // String litterals are not mutable, they are hardcoded values !
  
  let mut s = String::from("Look at me !");
  s.push_str(" See i am mutable");

  println!("{s}");
  {
    let mut s = "3";
    s = "We can only replace this value during compile";
  }

  // to push
  // The memory must be requested from the memory allocator at runtime.
  // String::from, its implementation requests the memory it needs.
  
  // We need a way of returning this memory to the allocator when we’re done with our String.

  // Rust takes a different path: the memory is automatically returned once the variable that owns it goes out of scope. 

  // when a memory goes out of scope, rust calls a function called "drop(..)"

  // This can have have odd effects sometimes
  let x = 5;
  let y = x;
  /*
  “bind the value 5 to x; 
  then make a copy of the value in x and bind it to y.” 
  We now have two variables, x and y, and both equal 5. 
  This is indeed what is happening, because integers are simple values with a known, 
  fixed size, and these two 5 values are pushed onto the stack. 

   If a type implements the Copy trait, variables that use it do not move, 
   but rather are trivially copied, making them still valid after assignment to another variable.
   Such is the case for integers, whose size is known at compile time
  */

  // What about values like string ? 
  let s1 = String::from("I sure hope no one will steal my string");
  let s2 = s1;                    // after this line, in order to not call drop on s1 again
  //                                         the reference s1 is considered no longer valid
  //                                         we call this a move (shallow copy + ref. becomes invalid)

  // println!("{s1}"); // This will provoke an error since s1 is invalid
  println!("s2: {s2}");

  // To DEEP COPY a string we can use the common methode clone(..)
  {
    let s1 = String::from("I sure hope no one copies my string");
    let s2 = s1.clone();
    println!("s1 : {s1}, s2 : {s2}");
  }


  // Be carefull when using functions ! 
  println!("{s2}");
  takes_ownership(s2); // s2 is now no longer valid, since the value was copied to *some_string*
  
  makes_copy(3);


  // Returning values can also transfer ownership
  println!("Look what I stole: {}", gives_ownership());

}



fn takes_ownership(some_string: String) { // some_string comes into scope
  println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
// memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
  println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

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

fn calculate_length(s: &String) -> usize {
  s.len()
}