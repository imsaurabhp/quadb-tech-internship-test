use std::io;

fn main() {
    println!("Enter a string to check if Palindrome");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read line");
    if(chk_pal(&input.trim()) == true){
        println!("Palindrome");
    }
    else{
        println!("Not a Palindrome");
    }
}

fn chk_pal(value: &str) -> bool {
    if value.len() < 2 {
        return false;
    }

    let mut start = 0;
    let mut end = value.len() - 1;
    let char_vector: Vec<char> = value.chars().collect();
    
    while start <= end {
        if char_vector.get(start) != char_vector.get(end) {
            return false;
        }
        start += 1;
        end -= 1;
    }
    return true;
}