use std::io::{self, BufRead};

fn check_arr(arr: &Vec<u32>) -> bool {
    for pair in arr.windows(2) {
        // [1, 2, 3, 4]
        // => [(0, (1, 2)), (1, (2, 3)), (2, (3, 4))]
        // => [(2, (3, 4)), (1, (2, 3)), (0, (1, 2))]
        // println!("{}, ({} {})", i, pair[0], pair[1]);
        if pair[0] > pair[1] {
            return false;
        }
    }
    true
}

fn do_a_case() {
    let stdin = io::stdin();
    let _n = stdin.lock().lines().next().unwrap().unwrap();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let mut arr: Vec<u32> = line
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    while !check_arr(&arr) {
        let mut bad_index = -1i32;
        for (i, pair) in arr.windows(2).enumerate().rev() {
            // [1, 2, 3, 4]
            // => [(0, (1, 2)), (1, (2, 3)), (2, (3, 4))]
            // => [(2, (3, 4)), (1, (2, 3)), (0, (1, 2))]
            // println!("{}, ({} {})", i, pair[0], pair[1]);
            if pair[0] > pair[1] {
                bad_index = i as i32;
                break;
            }
        }
        if bad_index != -1 {
            let minimum = arr[(bad_index + 1) as usize];
            for i in 0..bad_index {
                if arr[i as usize] > minimum {
                    arr[i as usize] = 0;
                }
            }
        }
    }
    println!(
        "{}",
        arr.iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}

fn main() {
    let stdin = io::stdin();
    let cases = stdin.lock().lines().next().unwrap().unwrap();
    let cases = cases.parse::<usize>().unwrap();
    for _ in 0..cases {
        do_a_case();
    }
}
