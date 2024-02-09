use std::io;

fn main() {
    println!("Enter array of integers (space-separated):");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let integer_vectors: Vec<i32> = input
        .trim()
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();
    let vector_len = integer_vectors.len(); // Use usize instead of i32
    println!("Enter k index");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let num: usize = input.trim().parse().expect("Invalid Input"); // Change the type to usize
    
    // println!("Array --> {:?} \nIndex k --> {}",integer_vectors, num);
    k_position_element(integer_vectors, num, vector_len);
}

fn k_position_element(integer_vectors: Vec<i32>, num: usize, arr_len: usize) {
    let mut sorted_vectors = integer_vectors.clone();
    sorted_vectors.sort();

    if num < 1 || num > arr_len {
        println!("Invalid k provided");
    } else {
        println!("Element {} found as the kth smallest element", sorted_vectors[num - 1]);
    }
}
