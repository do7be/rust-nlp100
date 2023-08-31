use std::fs;
use std::io::Write;
use std::path::Path;

struct PopularNames {
    name: String,
    gender: String,
    number: usize,
    year: usize,
}

fn main() {
    let str = fs::read_to_string("./data/popular-names.txt").unwrap();

    let mut result = str
        .lines()
        .map(|v| {
            let a = v
                .split('\t')
                .map(|v| v.to_string())
                .collect::<Vec<String>>();
            PopularNames {
                name: a[0].to_string(),
                gender: a[1].to_string(),
                number: a[2].parse().unwrap(),
                year: a[3].parse().unwrap(),
            }
        })
        .collect::<Vec<PopularNames>>();

    result.sort_by(|a, b| b.number.cmp(&a.number));

    let path = Path::new("./result.txt");
    let mut file = fs::File::create(path).unwrap();
    file.write_all(
        result
            .into_iter()
            .map(|v| vec![v.name, v.gender, v.number.to_string(), v.year.to_string()].join("\t"))
            .collect::<Vec<String>>()
            .join("\n")
            .as_bytes(),
    )
    .unwrap();
}
