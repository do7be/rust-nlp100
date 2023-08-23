fn main() {
    let str = "Now I need a drink, alcoholic of course, after the heavy lectures involving quantum mechanics.";
    let mut s = String::from(str);
    s.retain(|c| !r#",."#.contains(c));
    let splitted = s.split(' ').collect::<Vec<&str>>();

    let result = splitted
        .iter()
        .map(|v| v.chars().count())
        .collect::<Vec<usize>>();

    println!("{:?}", result);
}
