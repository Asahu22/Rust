fn max_subarray_sum(arr: &[i32]) -> i32 {
    if arr.is_empty() {
        return 0;
    }

    let mut max_current = arr[0];
    let mut max_global = arr[0];

    for &num in arr.iter().skip(1) {
        max_current = i32::max(num, max_current + num);
        if max_current > max_global {
            max_global = max_current;
        }
    }

    max_global
}

fn main() {
    let arr = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
    let max_sum = max_subarray_sum(&arr);

    println!("The maximum subarray sum is: {}", max_sum);
}
