use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("./data/jawiki-uk.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let start_reg = Regex::new(r#"^\{\{基礎情報"#).unwrap();
    let end_reg = Regex::new(r#"^\}\}"#).unwrap();
    let reg = Regex::new(r#"\|(.+) *= *(.+)"#).unwrap();

    let mut result = HashMap::new();

    let start = contents
        .lines()
        .position(|v| start_reg.is_match(v))
        .unwrap();
    let end = contents.lines().position(|v| end_reg.is_match(v)).unwrap();
    let contents = contents.lines().collect::<Vec<&str>>()[(start + 1)..end]
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<String>>();

    for v in &contents {
        let capture = match reg.captures(v) {
            Some(cap) => cap,
            None => continue,
        };

        let key = capture.get(1).unwrap().as_str();
        let field = capture.get(2).unwrap().as_str();
        result.insert(key, field);
    }

    println!("{:?}", result);
}
