use std::io;
fn main() {
    let mut number_str = String::new();
    println!("Please enter an integer number: ");
    io::stdin()
        .read_line(&mut number_str)
        .expect("Failed to read line!");
    
    
    let number: i32 = number_str.trim().parse().expect("Invalid input");
    //println!("{}",number);
    is_prime(number);
}
fn is_prime(num: i32)->bool{
    if num <= 1 {
        return false;
    }
    for a in 2..num{
        if num % a == 0 {
            println!("{} is not a prime number!", num);
            return false;
        }
    }
    println!("{} is a prime number", num);
    true
}



