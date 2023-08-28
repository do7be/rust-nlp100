fn main() {
    let str = include_str!("../../data/popular-names.txt");

    let result = str.lines().count();

    println!("{}", result);
}
