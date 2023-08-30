use std::collections::HashSet;
use std::fs;
use std::io::Write;
use std::path::Path;

fn main() {
    let str = fs::read_to_string("./data/popular-names.txt").unwrap();

    let result = str
        .lines()
        .map(|v| v.split('\t').collect::<Vec<&str>>()[0].to_string())
        .collect::<HashSet<String>>()
        .into_iter()
        .collect::<Vec<String>>();

    let path = Path::new("./result.txt");
    let mut file = fs::File::create(path).unwrap();
    file.write_all(result.join("\n\n").as_bytes()).unwrap();
}
