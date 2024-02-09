use std::io;

fn main() {
    println!("Enter a number");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let num_input: i32 = input.trim().parse().expect("Invalid Input");
    check_prime(num_input);
}

fn check_prime(num: i32) {
    if num <= 0 {
        println!("Not a Prime Number");
        return;
    }
    let mut count = 0;
    for i in 1..((num/2) + 1){
        if (num % i) == 0 {
            count += 1;
        }
    }
    
    if count > 1 {
        println!("Not a Prime Number");
    }
    else {
        println!("Prime Number");
    }
    return;
}