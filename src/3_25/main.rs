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

    let mut result: HashMap<&str, String> = HashMap::new();

    let start = contents
        .lines()
        .position(|v| start_reg.is_match(v))
        .unwrap();
    let end = contents.lines().position(|v| end_reg.is_match(v)).unwrap();
    let contents = contents.lines().collect::<Vec<&str>>()[(start + 1)..end]
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<String>>();

    let mut previous_key = "";
    for v in &contents {
        match reg.captures(v) {
            Some(cap) => {
                let key = cap.get(1).unwrap().as_str();
                let field = cap.get(2).unwrap().as_str();
                result.insert(key, field.to_string());
                previous_key = key;
            }
            None => {
                let field = result.get(previous_key).unwrap().to_string() + "\n" + v;
                result.insert(previous_key, field);
            }
        };
    }

    println!("{:?}", result);
}
