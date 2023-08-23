fn main() {
    let str = "stressed";
    let chars = str.chars();
    let result = chars
        .into_iter()
        .fold(String::new(), |acc, v| v.to_string() + &acc);

    println!("{}", result);
}
