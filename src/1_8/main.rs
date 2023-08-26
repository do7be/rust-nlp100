fn main() {
    let str = "Mb mznv rh wl7yv."; // My name is do7be.
    let result = cipher(str);
    println!("{}", result);
}

fn cipher(str: &str) -> String {
    str.chars()
        .map(|v| {
            if v.is_alphabetic() && v.is_lowercase() {
                let ascii = 219u32 - v as u32;
                let u8 = u8::try_from(ascii).unwrap();
                u8 as char
            } else {
                v
            }
        })
        .collect::<String>()
}
