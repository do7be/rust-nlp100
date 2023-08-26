use std::collections::HashSet;

fn main() {
    let str1 = "paraparaparadise";
    let str2 = "paragraph";

    let n = 2;
    println!("str1: {}", str1);
    println!("str2: {}", str2);
    println!("n: {}", n);
    println!("-");

    // 文字n-gram(X)
    let target_chars = str1.chars().map(|v| v.to_string()).collect::<Vec<String>>();
    let x = n_gram(target_chars, n);

    // 文字n-gram(Y)
    let target_chars = str2.chars().map(|v| v.to_string()).collect::<Vec<String>>();
    let y = n_gram(target_chars, n);

    println!("char {}-gram", n);
    println!("X: {:?}", x);
    println!("Y: {:?}", y);
    println!("-");

    let union = [x.clone(), y.clone()]
        .concat()
        .into_iter()
        .collect::<HashSet<String>>();
    let intersection = x
        .iter()
        .filter(|v| y.contains(v))
        .collect::<Vec<&String>>()
        .into_iter()
        .collect::<HashSet<&String>>();
    let difference = [
        x.iter()
            .filter(|v| !y.contains(v))
            .collect::<Vec<&String>>(),
        y.iter()
            .filter(|v| !x.contains(v))
            .collect::<Vec<&String>>(),
    ]
    .concat()
    .into_iter()
    .collect::<HashSet<&String>>();

    println!("Union: {:?}", union);
    println!("Intersection: {:?}", intersection);
    println!("Difference: {:?}", difference);
    println!("-");

    let exist_se_x = x.contains(&String::from("se"));
    let exist_se_y = y.contains(&String::from("se"));

    println!("exist \"se\" in X: {:?}", exist_se_x);
    println!("exist \"se\" in Y: {:?}", exist_se_y);
}

fn n_gram(target: Vec<String>, n: usize) -> Vec<String> {
    let mut result: Vec<String> = vec![];

    for i in 0..(target.len() - n + 1) {
        let v = &target[i..(i + n)];
        let a = v.join("");
        result.push(a);
    }

    result
}
