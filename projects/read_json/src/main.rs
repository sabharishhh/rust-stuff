use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Paragraph {
    name: String,

}

#[derive(Serialize, Deserialize)]
struct Article {
    article: String,
    author: String,
    paragraph: Vec<Paragraph>,
}

fn read_json_typed(raw_json: &str) -> Article {
    let parsed: Article = serde_json::from_str(raw_json).unwrap();
    return parsed;
}

fn main() {
    let json = r#"
        {
            "article": "how to work with json in rust",
            "author": "ayesha ahmed",
            "paragraph": [
                {
                    "name": "starting sentence"
                },
                {
                    "name": "body of the paragraph"
                },
                {
                    "name": "end of the paragraph"
                }
            ]
        }"#;

    let parsed: Article = read_json_typed(json);

    println!("\n\nThe name of the first paragraph is: {}", parsed.paragraph[0].name);
    println!("The name of the second paragraph is: {}", parsed.paragraph[1].name);
    println!("The name of the third paragraph is: {}", parsed.paragraph[2].name);
    println!("\nThe name of the author is: {}", parsed.author);

    println!();
}
