/*
Ownership:
each value stored has an owner (only one owner at a time). when the owner goes out of scope, the owned data will be trashed. There will be no memory leak.
set of rules that govern how a rust program manages memory. there is no garbage collection as in java or specific memory allocation/de-allocation strategies as in C. Rust uses the system of ownership where every program specifies a set of rules related to memory management which the compiler checks.
The program doesnt compile if the rules dont check. None of the rules affect the execution speed of the program.

The heap can only accept a single owner at any given point of time.
e.g. s1 = "hello"; s1 = s2; doesnt create another copy of s1 and point s2 to the start of that new copy, instead it points s2 to s1 itself. Now when s1 alters the content or flushes it out, s2 becomes the useless reference.
The problem of dangling references or double free pointers are caused by similar assignments.
In that case when s2 = s1 is executed, s1 is no longer valid and s2 takes charge. This prevents dangling reference.
It also avoids the expensive copy operation that needs to happen within the heap memory. 

Whenever a variable goes of out scope, whatever it owned also goes out of scope.

to create an independent copy of the non-trivial data item held by a variable, cloning is to be done as RUST by default only moves data items by transfer of ownership.
*/

fn main() {
    let s1: String = String::from("Hello world");
    println!("s1: {}", s1);
    
    let s2 = s1;

    //println!("s1: {}", s1); // prints an error and prgm doesnt compiler
    println!("s1: {}", s2); // is valid as s2 is the new pointer to "Hello World" prev referenced by s1

    let s: String = String::from("Hello string");
    let some_string: String = takes_ownership(s);
    // println!("{}", s); // would cause error as some_string has taken ownership of s the moment takes_ownership(<str>) is called
    
    // to pass the ownership and get the data back:
    let mut s: String = String::from("get back");
    s = takes_ownership(s);
    println!("{}", s); // now s: String will contain the data it passed to some_string via takes_ownership(<str>) but at the cost of takes_ownership(<str>) having to return the data back to the caller
    // clone function can also be used to pass the data without losing ownership
}

fn takes_ownership(some_string: String) -> String {
    println!("{}", some_string);
    return some_string;
}
