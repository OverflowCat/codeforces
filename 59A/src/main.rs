use std::io::{self, BufRead};
fn main() {
    let stdin = io::stdin();
    let word = stdin.lock().lines().next().unwrap().unwrap();
    let mut lowercase_count = 0;
    let mut uppercase_count = 0;
    for b in word.bytes() {
        if b >= b'a' && b <= b'z' {
            lowercase_count += 1;
        } else {
            uppercase_count += 1;
        }
    }
    println!(
        "{}",
        if uppercase_count > lowercase_count {
            word.to_uppercase()
        } else {
            word.to_lowercase()
        }
    );
}
