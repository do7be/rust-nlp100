fn main() {
    let str = "パタトクカシーー";
    let chars = str.chars().collect::<Vec<char>>();

    let result = chars
        .iter()
        .enumerate()
        .filter(|(i, _)| i % 2 == 0)
        .map(|(_, v)| v.to_string())
        .collect::<Vec<String>>()
        .join("");

    println!("{}", result);
}
