use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let line1 = stdin.lock().lines().next().unwrap().unwrap();
    let number = line1.parse::<u32>().unwrap();
    for _ in 0..number {
        let word = stdin.lock().lines().next().unwrap().unwrap();
        let length = word.len();
        // println!("The word contains {} characters", length);
        if length > 10 {
            println!("{}{}{}", &word[0..1], length - 2, &word[length - 1..length]);
        } else {
            println!("{}", word);
        }
    }
}
