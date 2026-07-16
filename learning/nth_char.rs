

fn main() {
    let greeting: String = String::from("hello w or ld");
    println!("{}", greeting);

    let char1: Option<char> = greeting.chars().nth(1000);

    match char1 {
        Some(c) => println!("{}", c),
        None => println!("No character at index 1000"),
    }

}
