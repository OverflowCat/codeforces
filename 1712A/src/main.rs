use std::{
    collections::HashMap,
    io::{self, BufRead},
};

fn do_a_case() {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let two_numbers = line.split(' ').collect::<Vec<&str>>();
    let (n, k) = (
        two_numbers[0].parse::<usize>().unwrap(),
        two_numbers[1].parse::<usize>().unwrap(),
    );
    let array = stdin.lock().lines().next().unwrap().unwrap();
    let array = array.split(' ').collect::<Vec<&str>>();
    let array = array
        .iter()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let mut sorted_array = array.clone();
    sorted_array.sort();
    // 把前 k 个数据的个数放到 hashmap 中
    let mut hashmap = HashMap::new();
    for i in 0..k {
        let count = hashmap.entry(sorted_array[i]).or_insert(0);
        *count += 1;
    }
    // println!("{:?}, {:?}", sorted_array, hashmap);

    // 对于 array[0..k]，不在 hashmap 里的移出去（o times）；
    // 对于 array[k..n]，在 hashmap 里的移进来（i times）。
    // 则总共需要 swap 的次数为 max(i, o)。
    let mut o: u32 = 0;
    for x in array[0..k].iter() {
        if !hashmap.contains_key(x) {
            o += 1;
        } else {
            // 从 hashmap 中减 1
            let count = hashmap.entry(*x).or_insert(0);
            if *count == 0 {
                panic!("count is 0");
            }
            *count -= 1;
            if *count == 0 {
                hashmap.remove(x);
            }
        }
    }

    let mut i: u32 = 0; // 还需要移进的个数
    for (_key, count) in hashmap.iter() {
        // 看看 hashmap 中还有啥子
        if *count <= 0 {
            panic!("count is {} when checking items left in hashmap", count);
        }
        i += *count;
    }

    println!("{}", std::cmp::max(i, o));
    return;
}

fn main() {
    let stdin = io::stdin();
    let cases = stdin.lock().lines().next().unwrap().unwrap();
    let cases = cases.parse::<usize>().unwrap();
    for _ in 0..cases {
        do_a_case();
    }
}
