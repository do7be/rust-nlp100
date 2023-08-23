fn main() {
    let str = "パタトクカシーー";
    let chars = str.chars();

    let result = chars
        .enumerate()
        .filter(|(i, _)| i % 2 == 0)
        .map(|(_, v)| v.to_string())
        .collect::<String>();

    println!("{}", result);
}
