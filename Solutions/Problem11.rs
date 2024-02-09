use std::io;

fn main() {
    println!("Enter first sorted array of integers (space-separated):");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let first_sorted_array: Vec<i32> = input
        .trim()
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();
    println!("Enter second sorted array of integers (space-separated):");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let second_sorted_array: Vec<i32> = input
        .trim()
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();
    combine_arrays(first_sorted_array, second_sorted_array);

}

fn combine_arrays(mut arr1: Vec<i32>, mut arr2: Vec<i32>) {
    arr1.append(&mut arr2);
    arr1.sort_by(|a, b| a.cmp(&b));
    println!("Sorted & Merged Array {:?}", arr1);
}