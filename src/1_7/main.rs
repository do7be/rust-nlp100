fn main() {
    let x = 12;
    let y = "気温";
    let z = 22.4;
    let result = struct_date_temp(x, y, z);
    println!("{}", result);
}

fn struct_date_temp(x: u8, y: &str, z: f32) -> String {
    format!("{}時の{}は{}", x, y, z)
}
