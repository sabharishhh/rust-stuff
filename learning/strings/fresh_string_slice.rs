// fresh string slices

fn main() {
    let name = String::from("Hello world");
    let ans = first_word(&name); // borrowed and not an ownership transfer

    println!("ans is {}", ans);
    println!("{}", name);
}

fn first_word(str: &String) -> String {
    let mut ans = String::from("");

    for i in str.chars() {
        if i == ' ' {
            break;
        }
        ans.push_str(&i.to_string());
    }

    return ans;
}

// problems with the above implementation are:
// it takes double the memory ie., name points to "hello world" and ans separately points to "hello"
// when name gets cleared, ans variable will still point to "hello" as "hello" is not yet cleared