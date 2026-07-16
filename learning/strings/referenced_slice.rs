// referenced slices

fn main() {
    let mut word = String::from("Hello world");
    let word1 = &word[0..5];

    // word.push_str(", welcome to RUST programming"); 
    // word.clear();
    // these arent allowed as there is a mutable reference to word held by word1 that is still in-scope. if word.clear() executes, word1 becomes a dangling pointer

    println!("{}", word1);
}

