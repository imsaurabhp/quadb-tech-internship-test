use std::io;

fn main() {
    println!("Enter a string to reverse");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let reversed = reverse(&input);
    println!("{}", reversed);

}

fn reverse(s: &str) -> String {
    s.chars().rev().collect()
}