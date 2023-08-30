use std::fs;
use std::io::Write;
use std::path::Path;

fn main() {
    let str = fs::read_to_string("./data/popular-names.txt").unwrap();

    let mut result = str
        .lines()
        .map(|v| {
            v.split('\t')
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>();

    // TODO: refactoring
    result.sort_by(|a, b| {
        b[2].parse::<usize>()
            .unwrap()
            .cmp(&a[2].parse::<usize>().unwrap())
    });

    let path = Path::new("./result.txt");
    let mut file = fs::File::create(path).unwrap();
    file.write_all(
        result
            .into_iter()
            .map(|v| v.join("\t"))
            .collect::<Vec<String>>()
            .join("\n")
            .as_bytes(),
    )
    .unwrap();
}
