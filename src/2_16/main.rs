use std::fs;
use std::io;
use std::io::Write;
use std::path::Path;

fn main() {
    print!("n: ");
    io::stdout().flush().unwrap();

    let n: usize = {
        let mut input = String::new();
        let _ = io::stdin().read_line(&mut input);
        input.trim_end().parse().unwrap()
    };

    let str = fs::read_to_string("./data/popular-names.txt").unwrap();

    let lines = str.lines().map(|v| v.to_string()).collect::<Vec<String>>();
    let result = lines
        .chunks(n)
        .map(|chunk| chunk.join("\n"))
        .collect::<Vec<String>>();

    let path = Path::new("./result.txt");
    let mut file = fs::File::create(path).unwrap();
    file.write_all(result.join("\n\n").as_bytes()).unwrap();
}
