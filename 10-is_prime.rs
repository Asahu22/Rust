fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..=(n as f64).sqrt() as u32 {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn main() {
    let num_to_check = 17; 
    if is_prime(num_to_check) {
        println!("{} is prime!", num_to_check);
    } else {
        println!("{} is not prime.", num_to_check);
    }
}
