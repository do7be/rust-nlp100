use std::collections::HashMap;

fn main() {
    let str = "Hi He Lied Because Boron Could Not Oxidize Fluorine. New Nations Might Also Sign Peace Security Clause. Arthur King Can.";
    let first_str_index_list: [usize; 9] = [0, 4, 5, 6, 7, 8, 14, 15, 18];

    let mut s = String::from(str);
    s.retain(|c| !r#",."#.contains(c));
    let splitted = s.split(' ').collect::<Vec<&str>>();

    let mut result = HashMap::new();

    for (i, v) in splitted.iter().enumerate() {
        let k = if first_str_index_list.contains(&i) {
            &v[..1]
        } else {
            &v[0..=1]
        };
        result.insert(k, i + 1);
    }

    println!("{:?}", result);
}
