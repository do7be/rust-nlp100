use std::fs;
use std::io::Write;
use std::path::Path;

fn main() {
    let str = fs::read_to_string("./data/popular-names.txt").unwrap();

    let mut result = str
        .lines()
        .map(|v| {
            let a = v
                .split('\t')
                .map(|v| v.to_string())
                .collect::<Vec<String>>();
            (
                a[0].to_string(),
                a[1].to_string(),
                a[2].parse().unwrap(),
                a[3].parse().unwrap(),
            )
        })
        .collect::<Vec<(String, String, usize, usize)>>();

    result.sort_by(|a, b| b.2.cmp(&a.2));

    let path = Path::new("./result.txt");
    let mut file = fs::File::create(path).unwrap();
    file.write_all(
        result
            .into_iter()
            .map(|v| vec![v.0, v.1, v.2.to_string(), v.3.to_string()].join("\t"))
            .collect::<Vec<String>>()
            .join("\n")
            .as_bytes(),
    )
    .unwrap();
}
