// traits are like abstract classes in JAVA or Python or whatever OOP language it is
// they will have their own implementation or a default implementation which can optionally be overridden

pub trait Summary { // pub doesnt matter since everything is here only!
    fn summarize(&self) -> String;
}

struct User {
    name: String,
    age: u32,
}

impl Summary for User {
    fn summarize(&self) -> String {
        return format!("User: {} is {} years old.", self.name, self.age);
    }
}

fn main() {
    let user = User {
        name: String::from("Sabharish"),
        age: 22,
    };
    println!("{}", user.summarize());
}