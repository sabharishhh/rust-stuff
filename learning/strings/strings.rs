// strings vs slices:
// string type provided by rust std lib is growable, mutable, owned, 
// slices let you ref a contiguous section of elements in a collection rather than the whole collection

fn main() {
    // creating a string:
    let mut name = String::from("Sabharish");
    println!("Name is {}", name);

    // appending to a string:
    name.push_str(" P V");
    println!("Name is {}", name);

    // deleting from a string:
    name.replace_range(9..name.len(), "");
    println!("Name is {}", name);
    
}