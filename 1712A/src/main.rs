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
    let mut array = array
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

    let mut swap_count: u32 = 0;
    // 最小，则必是排好序的
    for i in 0..n {
        for j in 0..n - 1 - i {
            if array[j] > array[j + 1] {
                array.swap(j, j + 1);
                swap_count += 1;
                // println!("std {:?}", array);
                // println!("arr {:?}, swap {}x", array, swap_count);
                // println!("j is {}, j+1 is {}, k is {}", j, j + 1, k);
                if j <= k {
                    // 交换的数在前面
                    let mut temp_hashmap = HashMap::new();
                    for i in 0..k {
                        let count = temp_hashmap.entry(array[i]).or_insert(0);
                        *count += 1;
                    }
                    if temp_hashmap == hashmap {
                        println!("{}", swap_count / 2);
                        return;
                    } else {
                        // println!("temp hashmap {:?}", temp_hashmap);
                    }
                }
            }
        }
    }
    println!("0");
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
