use std::io;

fn main() {
    println!("Enter a string of words (space-separated):");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let words_vector: Vec<String> = input.trim().split_whitespace().map(|s| s.to_string()).collect();
    smallest_string(words_vector);
}

fn smallest_string(input_vector: Vec<String>){
    let sorted_words_vector = sort_vector(input_vector);
    if sorted_words_vector.len() > 0 {
        println!("{}",sorted_words_vector[0]);
    }
    else{
        println!("Please provide valid input");
    }
}

fn sort_vector(input: Vec<String>) -> Vec<String> {
    let mut sorted_vector = input.clone(); // Cloning to avoid modifying the original vector
    sorted_vector.sort_by(|a, b| a.len().cmp(&b.len()));
    return sorted_vector;
}