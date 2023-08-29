use std::fs;
use std::io;
use std::io::Write;

fn main() {
    print!("n: ");
    io::stdout().flush().unwrap();

    let n: usize = {
        let mut input = String::new();
        let _ = io::stdin().read_line(&mut input);
        input.trim_end().parse().unwrap()
    };

    let str = fs::read_to_string("./data/popular-names.txt").unwrap();

    let result = str.lines().collect::<Vec<&str>>()[0..n]
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<String>>();

    println!("{}", result.join("\n"));
}
