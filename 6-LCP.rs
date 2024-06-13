// /Implement a function that finds the longest common prefix of a given set of strings.
use std::cmp::Ordering;

fn longest_common_prefix(arr: &[String]) -> String {
    if arr.is_empty() {
        return String::new();
    }

    if arr.len() == 1 {
        return arr[0].clone();
    }

    let mut sorted_arr = arr.to_vec();
    sorted_arr.sort();

    let first = sorted_arr.first().unwrap();
    let last = sorted_arr.last().unwrap();

    let mut i = 0;
    while i < first.len() && first.as_bytes()[i] == last.as_bytes()[i] {
        i += 1;
    }

    first.get(..i).unwrap().to_string()
}

fn main() {
    let arr = vec![
        String::from("cold"),
        String::from("coldhunter"),
        String::from("colder"),
        String::from("coldeee"),
    ];
    println!("The longest common prefix is: {}", longest_common_prefix(&arr));
}

