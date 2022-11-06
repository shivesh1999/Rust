use std::io;
use rand;
fn main() {
    let mut buffer = String::new();
    println!("Enter your name :");
    io::stdin().read_line(&mut buffer);// User Input 
    println!("Hello {}",buffer);
    let number = rand::random::<u16>();
    println!("Your random number is {}",number);
}
