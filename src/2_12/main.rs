use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    let str = include_str!("../../data/popular-names.txt");

    let line_strs = str.lines().map(|v| v.split('\t').collect::<Vec<&str>>());
    let col1 = line_strs.clone().map(|v| v[0]).collect::<Vec<&str>>();
    let col2 = line_strs.clone().map(|v| v[1]).collect::<Vec<&str>>();

    let col1_path = Path::new("./data/col1.txt");
    let col2_path = Path::new("./data/col2.txt");
    let mut file1 = File::create(col1_path).unwrap();
    let mut file2 = File::create(col2_path).unwrap();
    file1.write_all(col1.join("\n").as_bytes()).unwrap();
    file2.write_all(col2.join("\n").as_bytes()).unwrap();
}
