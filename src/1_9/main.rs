extern crate rand;
use rand::Rng;

fn main() {
    let str = "I couldn’t believe that I could actually understand what I was reading : the phenomenal power of the human mind .";
    let target = String::from(str)
        .split(' ')
        .map(|v| v.to_string())
        .collect::<Vec<String>>();
    let result = shuffle(target);
    println!("{}", result.join(" "));
}

fn shuffle(target: Vec<String>) -> Vec<String> {
    target
        .iter()
        .map(|v| {
            if v.len() >= 5 {
                let chars = v.chars().collect::<Vec<char>>();
                let mut rng = rand::thread_rng();
                let mut shuffle_chars = chars[1..(chars.len() - 1)].to_vec();
                rng.shuffle(&mut shuffle_chars);

                let mut result: Vec<char> = Vec::new();
                result.extend(vec![chars[0]]);
                result.extend(shuffle_chars);
                result.extend(vec![chars[chars.len() - 1]]);
                result.iter().collect::<String>()
            } else {
                v.to_string()
            }
        })
        .collect::<Vec<String>>()
}
