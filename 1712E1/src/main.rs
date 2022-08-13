use std::{
    collections::HashMap,
    io::{self, BufRead},
};

fn main() {
    let mut calculated_lcms: HashMap<(u64, u64), u64> = HashMap::new();
    let mut calculated_gcds: HashMap<(u64, u64), u64> = HashMap::new();
    let mut checked_triplets: HashMap<(u64, u64, u64), bool> = HashMap::new();

    let mut gcd = |a: u64, b: u64| -> u64 {
        match calculated_gcds.get(&(a, b)) {
            Some(v) => v.clone(),
            None => {
                let mut max = a;
                let mut min = b;
                if min > max {
                    let val = max;
                    max = min;
                    min = val;
                }

                loop {
                    let res = max % min;
                    if res == 0 {
                        calculated_gcds.insert((a, b), min);
                        return min;
                    }
                    max = min;
                    min = res;
                }
            }
        }
    };

    let mut lcm = |a: u64, b: u64| match calculated_lcms.get(&(a, b)) {
        Some(v) => v.clone(),
        None => {
            let result = if a + 1 == b { a * b } else { a * b / gcd(a, b) };
            calculated_lcms.insert((a, b), result);
            result
        }
    };

    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let cases: usize = line.parse::<usize>().unwrap();
    for _ in 0..cases {
        let (l, r) = {
            let line = stdin.lock().lines().next().unwrap().unwrap();
            let line = line.split(' ').collect::<Vec<&str>>();
            (
                line[0].parse::<u64>().unwrap(),
                line[1].parse::<u64>().unwrap(),
            )
        };

        let mut count = 0usize;
        for i in l..=r {
            for j in i + 1..=r {
                for k in j + 1..=r {
                    match checked_triplets.get(&(i, j, k)) {
                        Some(v) => {
                            if *v {
                                count += 1;
                            }
                        }
                        None => {
                            let lcm_of_i_j = lcm(i, j);
                            let lcm_of_i_j_k = lcm(lcm_of_i_j, k);
                            if lcm_of_i_j_k >= i + j + k {
                                checked_triplets.insert((i, j, k), true);
                                count += 1;
                            } else {
                                checked_triplets.insert((i, j, k), false);
                            }
                        }
                    }
                }
            }
        }
        println!("{}", count);
    }
}
