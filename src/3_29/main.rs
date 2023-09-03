use regex::Regex;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Deserialize)]
struct ImageInfo {
    url: String,
    descriptionurl: String,
    descriptionshorturl: String,
}

#[derive(Debug, Deserialize)]
struct Page {
    ns: i32,
    title: String,
    missing: Option<String>,
    known: Option<String>,
    imagerepository: String,
    imageinfo: Vec<ImageInfo>,
}

#[derive(Debug, Deserialize)]
struct Query {
    pages: std::collections::BTreeMap<String, Page>,
}

#[derive(Debug, Deserialize)]
struct ResponseBody {
    query: Query,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let mut file = File::open("./data/jawiki-uk.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let template = get_template(&contents);
    let removed_emphasis = remove_emphasis(template);
    let removed_links = remove_links(removed_emphasis);
    let removed_markups = remove_markups(removed_links);

    let file_name = removed_markups.get("国旗画像").unwrap();

    println!("国旗画像：{}", file_name);

    let url = "https://www.mediawiki.org/w/api.php";
    let titles = "File:".to_string() + file_name;
    let params = [
        ("action", "query"),
        ("format", "json"),
        ("prop", "imageinfo"),
        ("iiprop", "url"),
        ("titles", titles.as_str()),
    ];
    let url = reqwest::Url::parse_with_params(url, &params).unwrap();
    let body = reqwest::get(url).await?.json::<ResponseBody>().await?;

    println!("{:?}", body.query.pages["-1"].imageinfo[0].url);

    Ok(())
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
