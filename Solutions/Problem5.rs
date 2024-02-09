use std::io;

fn main() {
    println!("Enter a sorted array of integers (space-separated):");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let sorted_array: Vec<i32> = input
        .trim()
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();
    let median: i32 = median_of_array(sorted_array);
    println!("{}", median);
}

fn median_of_array(sorted_arr: Vec<i32>) -> i32{
    let arr_len = sorted_arr.len();
    if arr_len == 0 {
        return -1;
    }
    else if arr_len == 1 {
        return sorted_arr[0];
    }
    else if arr_len % 2 == 0 {
        let mid_right_index = arr_len / 2;
        let mid_left_index = (arr_len / 2) - 1;
        return (sorted_arr[mid_left_index] + sorted_arr[mid_right_index]) / 2;
    }
    else {
        return sorted_arr[arr_len / 2];
    }
}