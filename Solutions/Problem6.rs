use std::io;

fn main() {
    println!("Enter a string of words (space-separated):");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let words_vector: Vec<String> = input.trim().split_whitespace().map(|s| s.to_string()).collect();
    println!("Longest Common Prefix: {}",longest_common_prefix(words_vector));
}

fn longest_common_prefix(input: Vec<String>) -> String {
    let sorted_vector = sort_vector(input);
    let max_len = sorted_vector[0].len() - 1; // Length of the smallest string
    let first_elem = sorted_vector[0].to_string();
    let second_elem = sorted_vector[1].to_string();
    let mut prefix = "";
    for i in 0..max_len {
        if &first_elem[0..i] != &second_elem[0..i] {
            break;
        }
        prefix = &first_elem[0..i];
    }
    return prefix.to_string();
}

fn sort_vector(input: Vec<String>) -> Vec<String> {
    let mut sorted_vector = input.clone(); // Cloning to avoid modifying the original vector
    sorted_vector.sort_by(|a, b| a.len().cmp(&b.len()));
    return sorted_vector;
}
