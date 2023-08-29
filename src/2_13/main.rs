use std::fs;
use std::io::Write;
use std::path::Path;

fn main() {
    let col1 = fs::read_to_string("./data/col1.txt").unwrap();
    let col2 = fs::read_to_string("./data/col2.txt").unwrap();

    let col1_lines = col1.lines().collect::<Vec<&str>>();
    let col2_lines = col2.lines().collect::<Vec<&str>>();

    let mut result = Vec::new();
    for i in 0..col1_lines.len() {
        result.push(col1_lines[i].to_string() + "\t" + col2_lines[i]);
    }

    let path = Path::new("./result.txt");
    let mut file = fs::File::create(path).unwrap();
    file.write_all(result.join("\n").as_bytes()).unwrap();
}
