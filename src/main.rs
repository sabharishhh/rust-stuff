// derived macros

#[derive(Debug)]

struct User {
    name: String,
    age: u32,
}

fn main() {
    let user = User {
        name: String::from("Sabharish"),
        age: 22,
    };

    println!("{:?}", user);
}