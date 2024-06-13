//Implement a function that checks whether a given number is prime or not.
use std::io;

fn is_prime(n:i32)->bool{
    if n<1 {
        return false;
    }

    for i in 2..=(n/2){
        if n%i==0{
            return false
        }
    }
    true
}

fn main(){
    let mut number =String::new();
    println!("Enter a number:");
    io::stdin().read_line(&mut number).expect("Failed to read line");
    let number: i32 = number.trim().parse().expect("Invalid number");

    if is_prime(number) {
        println!("{} is a prime number.", number);
    } else {
        println!("{} is not a prime number.", number);
    }
}