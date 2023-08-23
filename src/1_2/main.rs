fn main() {
    let str_p = "パトカー";
    let str_t = "タクシー";
    let mut chars_p = str_p.chars();
    let mut chars_t = str_t.chars();

    let mut result = String::new();
    let length = chars_p.clone().count();

    for _ in 0..length {
        result += &chars_p.next().unwrap().to_string();
        result += &chars_t.next().unwrap().to_string();
    }

    println!("{}", result);
}
