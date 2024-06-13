//Given a string of words, implement a function that returns the shortest word in the string.
use std::io::Cursor;

fn find_shortest_word(input: &str) -> String {
    let mut shortest_word = String::new();
    let mut shortest_length = usize::MAX; // Initialize with a large value

    for word in input.split_whitespace() {
        let word_length = word.chars().count();
        if word_length < shortest_length {
            shortest_word = word.to_string();
            shortest_length = word_length;
        }
    }

    shortest_word
}

fn main() {
    let input = "The quick brown fox jumps over the lazy dog";
    let shortest = find_shortest_word(input);
    println!("Shortest word: {}", shortest);
}
