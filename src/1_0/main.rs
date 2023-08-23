fn main() {
    let str = "stressed";
    let result = str
        .chars()
        .rev()
        .collect::<Vec<char>>()
        .iter()
        .collect::<String>();

    println!("{}", result);
}
