use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("./data/jawiki-uk.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let template = get_template(&contents);
    let removed_emphasis = remove_emphasis(template);
    let removed_links = remove_links(removed_emphasis);
    let result = remove_markups(removed_links);

    for (k, v) in result.iter() {
        println!("{}: {}", k, v);
    }
}

fn remove_markups(template: HashMap<String, String>) -> HashMap<String, String> {
    let mut result: HashMap<String, String> = HashMap::new();

    let reg = Regex::new(r#"([^<>]*)<[^<>]+>([^<>]*)"#).unwrap();
    for (key, value) in template.iter() {
        let str = reg
            .captures_iter(value)
            .map(|cap| cap.get(1).unwrap().as_str().to_string() + cap.get(2).unwrap().as_str())
            .collect::<Vec<String>>();

        result.insert(
            key.to_string(),
            if str.is_empty() {
                value.to_string()
            } else {
                str.join("")
            },
        );
    }
    result
}

fn remove_links(template: HashMap<String, String>) -> HashMap<String, String> {
    let mut result: HashMap<String, String> = HashMap::new();

    let reg = Regex::new(r#"([^\]]*)\[\[(?:[^\[\]]+\|)?([^\[]+)\]\]([^\[]*)"#).unwrap();
    for (key, value) in template.iter() {
        let str = reg
            .captures_iter(value)
            .map(|cap| {
                cap.get(1).unwrap().as_str().to_string()
                    + cap.get(2).unwrap().as_str()
                    + cap.get(3).unwrap().as_str()
            })
            .collect::<Vec<String>>();

        result.insert(
            key.to_string(),
            if str.is_empty() {
                value.to_string()
            } else {
                str.join("")
            },
        );
    }
    result
}

fn remove_emphasis(template: HashMap<String, String>) -> HashMap<String, String> {
    let mut result: HashMap<String, String> = HashMap::new();

    let reg = Regex::new(r#"([^']*)'{1,3}([^']+)'{1,3}([^']*)"#).unwrap();
    for (key, value) in template.iter() {
        let str = reg
            .captures_iter(value)
            .map(|cap| {
                cap.get(1).unwrap().as_str().to_string()
                    + cap.get(2).unwrap().as_str()
                    + cap.get(3).unwrap().as_str()
            })
            .collect::<Vec<String>>();

        result.insert(
            key.to_string(),
            if str.is_empty() {
                value.to_string()
            } else {
                str.join("")
            },
        );
    }
    result
}

fn get_template(contents: &str) -> HashMap<String, String> {
    let reg = Regex::new(r#"\|([^ ]+) *= *(.+)"#).unwrap();
    let start_reg = Regex::new(r#"^\{\{基礎情報"#).unwrap();
    let end_reg = Regex::new(r#"^\}\}"#).unwrap();

    let start = contents
        .lines()
        .position(|v| start_reg.is_match(v))
        .unwrap();
    let end = contents.lines().position(|v| end_reg.is_match(v)).unwrap();

    let contents = contents.lines().collect::<Vec<&str>>()[(start + 1)..end]
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<String>>();

    let mut result: HashMap<String, String> = HashMap::new();

    let mut previous_key = String::new();
    for v in &contents {
        match reg.captures(v) {
            Some(cap) => {
                let key = cap.get(1).unwrap().as_str().to_string();
                let field = cap.get(2).unwrap().as_str();
                result.insert(key.clone(), field.to_string());
                previous_key = key;
            }
            None => {
                let field = result.get(&previous_key).unwrap().to_string() + "\n" + v;
                result.insert(previous_key.clone(), field);
            }
        };
    }

    result
}
