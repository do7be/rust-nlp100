fn main() {
    let str = "I am an NLPer";
    let n = 2;
    println!("str {}", str);
    println!("n: {}", n);

    // 単語n-gram
    let mut s = String::from(str);
    s.retain(|c| !r#",."#.contains(c));
    let target_words = s.split(' ').map(|v| v.to_string()).collect::<Vec<String>>();
    let result_words = n_gram(target_words, n);
    println!("word {}-gram", n);
    println!("{:?}", result_words);

    // 文字n-gram
    let target_chars = str.chars().map(|v| v.to_string()).collect::<Vec<String>>();
    let result_chars = n_gram(target_chars, n);
    println!("char {}-gram", n);
    println!("{:?}", result_chars);
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
