/*
 References: only the address of string is passed instead of the ownership of the string data. &-ambersant is used.
 e.g. s = "hello"; s1 = &s; : here s1 points to s only, so no direct ownership transfer.

 Borrowing: pass the ownership of variables to functions by pasing reference to the string instead of the value and still retain the ownership of the string data.
 w.g. takes_ownership(&s1); instead of takes_ownership(s1);

 Mutable references: when a string variable is made mutable, then the mutable reference of that string can be used to update the string data by passing it to other functions.
 A mutable reference to a mutable string var can be first assigned and then this reference could be used to pass by ref to functions for data manipulation that gets reflected in the original mutable string variable also.

 If a variable has a mutable reference, then it cannot have another mutable or immutable reference simultaneously.
 many immutable references can be present but not the case for mutable ref due to race condition or inconsistent updates causing synchronization issues.
*/

fn main() {
    let mut s1: String = String::from("Hello world");
    s1.push_str(" to Rust programming");
    println!("{}", s1);

    update_str(&mut s1);
    println!("{}", s1);

    let mut s2: String = String::from("Hello");
    let s3: &mut String = &mut s2;

    update_str(s3);
    println!("{}", s2);
}

fn update_str(s: &mut String) {
    s.push_str(", it is a hard path.");
}