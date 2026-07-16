// slices

fn main() {
    let word = String::from("Hello world");
    let word1 = find_first_word(&word);

    println!("{}", word1);
}

fn find_first_word(word: &String) -> &str {
     let mut index = 0;

     for (_, i) in word.chars().enumerate() {
         if i == ' ' {
             break;
         }
         index += 1;
     }

     return &word[0..index];
 }

 // return &str which is a simple reference to the first word of the referenced string data