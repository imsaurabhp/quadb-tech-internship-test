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


    println!("Enter the integer to find first occurence");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let num: i32 = input.trim().parse().expect("Invalid Input");

    let position = get_first_occurence(&sorted_array, num);
    println!("{}",position);
}

fn get_first_occurence(sorted_array: &[i32], num: i32) -> i32 {
    let mut high = sorted_array.len() - 1;

    while low <= high {
        let mid = (low + high) / 2;
        if sorted_array[mid] == num{
            return find_first_occ_from_range(&sorted_array, low.try_into().unwrap(), high.try_into().unwrap(), num);
        }
        else if sorted_array[mid] <= num {
            low = mid + 1;
        }
        else {
            high = mid - 1;
        }
    }
    return -1
}


fn find_first_occ_from_range(sorted_array: &[i32], low: i32, high: i32, num: i32) -> i32{
    for x in low..(high+1){
        if sorted_array[x as usize] == num {
            return x;
        }
    }
    return -1;
}