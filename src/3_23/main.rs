use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("./data/jawiki-uk.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let reg = Regex::new(r"(={2,4})([^=]+)(={2,4})").unwrap();

    let mut result = HashMap::new();

    let contents = contents.lines().filter(|v| reg.is_match(v));

    for v in contents {
        let capture = reg.captures(v).unwrap();
        let label = capture.get(2).unwrap().as_str();
        let section_level = capture.get(1).unwrap().len();
        result.insert(label, section_level);
    }
    println!("{:?}", result);
}
