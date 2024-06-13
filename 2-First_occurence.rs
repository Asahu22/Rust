//Given a sorted array of integers, implement a function that returns the index of the first occurrence of a given number.
use std::collections::HashMap;

fn solve(n: i32, key: i32, v: &Vec<i32>) -> i32 {
    let mut start = 0;
    let mut end = n - 1;
    let mut res = -1;

    while start <= end {
        let mid = start + (end - start) / 2;
        if v[mid as usize] == key {
            res = mid;
            end = mid - 1;
        } else if key < v[mid as usize] {
            end = mid - 1;
        } else {
            start = mid + 1;
        }
    }
    res
}

fn main() {
    let n = 7;
    let key = 20;
    let v = vec![3, 4, 13, 13, 13, 20, 40];
    println!("The index of first occurance of the key value: {}", solve(n, key, &v));
}

