use regex::Regex;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("./data/jawiki-uk.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let reg = Regex::new(r"={2,4}([^=]+)={2,4}").unwrap();

    let result = contents
        .lines()
        .filter(|v| reg.is_match(v))
        .map(|v| {
            reg.captures(v)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .to_string()
        })
        .collect::<Vec<String>>();

    println!("{:?}", result);
}
