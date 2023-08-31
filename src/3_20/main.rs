use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
struct Article {
    title: String,
    text: String,
}

fn main() {
    let mut file = File::open("./data/jawiki-country.json").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let articles = contents
        .lines()
        .map(|line| serde_json::from_str::<Article>(line).unwrap())
        .filter(|article| article.title == "イギリス")
        .collect::<Vec<Article>>();

    let path = Path::new("./result.txt");
    let mut file = fs::File::create(path).unwrap();
    file.write_all(articles[0].text.as_bytes()).unwrap();
}
