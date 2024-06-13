//Given a sorted array of integers, implement a function that returns the median of the array.

use std::io;

fn find_median(array: &[i32]) -> f64 {
    let n = array.len();
    if n % 2 == 0 {
        // Array size is even
        let middle1 = array[n / 2 - 1];
        let middle2 = array[n / 2];
        (middle1 as f64 + middle2 as f64) / 2.0
    } else {
        // Array size is odd
        array[n / 2] as f64
    }
}

fn main() {
    let array = [1, 2, 3, 4, 5]; // Sorted array
    let median = find_median(&array);
    println!("Median of the Array: {}", median);
}

