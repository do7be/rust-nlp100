fn main() {
    let str = "stressed";
    let chars = str.chars().collect::<Vec<char>>();
    let result = chars
        .iter()
        .fold(String::new(), |acc, v| v.to_string() + &acc);

    println!("{}", result);
}
