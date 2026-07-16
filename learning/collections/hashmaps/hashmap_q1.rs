use std::collections::HashMap;

fn main() {
    let input_vec = vec![
        (String::from("Sabharish"), 21),
        (String::from("Harkirat"), 22),
    ];

    let map = group_values_by_keys(input_vec);
    println!("{:?}", map);
}

fn group_values_by_keys(vec: Vec<(String, i32)>) -> HashMap<String, i32> {
    let mut hashmap = HashMap::new();

    for (key, value) in vec {
        hm.insert(key, value);
    }
    
    return hashmap;
}