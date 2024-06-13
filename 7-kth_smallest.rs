//Implement a function that returns the kth smallest element in a given array.

use std::vec;

fn kth_smallest(arr: &mut Vec<i32>, k: usize) -> i32 {
    arr.sort();
    return arr[k - 1];
}

fn main() {
    let mut arr = vec![12, 3, 5, 7, 19];
    let n = arr.len();
    let k = 2;
    println!("K'th smallest element is: {}", kth_smallest(&mut arr, k));
}