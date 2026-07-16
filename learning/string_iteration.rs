fn main() {
    let sentence = String::from("my name is sabharish");
    
    let word = get_first_word(sentence);
    println!("{}", word);
}

fn get_first_word (sentence: String) -> String {
    let mut result = String::from("");

    for ch in sentence.chars() {
        result.push_str(&ch.to_string());
        
        if ch == ' ' {
            break;
        }
    }

    return result;
}