use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::Path;

fn main() {
    let str = fs::read_to_string("./data/popular-names.txt").unwrap();

    let col1 = str
        .lines()
        .map(|v| v.split('\t').collect::<Vec<&str>>()[0].to_string())
        .collect::<Vec<String>>();

    let mut counts: HashMap<String, usize> = HashMap::new();
    for c in col1.iter() {
        let count = counts.get(c);
        counts.insert(
            c.to_string(),
            match count {
                Option::Some(v) => *v + 1,
                Option::None => 1,
            },
        );
    }

    let mut result = counts.iter().collect::<Vec<(&String, &usize)>>();
    result.sort_by(|a, b| b.1.cmp(a.1));

    let path = Path::new("./result.txt");
    let mut file = fs::File::create(path).unwrap();
    file.write_all(
        result
            .iter()
            .map(|v| v.0.to_string() + "\t" + &v.1.to_string())
            .collect::<Vec<String>>()
            .join("\n")
            .as_bytes(),
    )
    .unwrap();
}
