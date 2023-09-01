use regex::Regex;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("./data/jawiki-uk.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let reg = Regex::new(r#"ファイル:([^|\}\]]+\.[^|\}\]]+)"#).unwrap();

    let result = reg
        .captures_iter(&contents)
        .map(|cap| cap.get(1).unwrap().as_str().to_string())
        .collect::<Vec<String>>();

    println!("{}", result.join("\n"));
    println!("{}", result.len());
}
