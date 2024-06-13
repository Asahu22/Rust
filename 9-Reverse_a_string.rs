//Reverse a string in Rustv

fn reverse(s: &str) -> String {
    s.chars().rev().collect()
}

fn main() {
    let original = "hello";
    let reversed = reverse(original);
    println!("Original: {}", original);
    println!("Reversed: {}", reversed);
}
