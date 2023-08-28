fn main() {
    let str = include_str!("../../data/popular-names.txt");

    let result = str.replace('\t', " ");

    println!("{}", result);
}
