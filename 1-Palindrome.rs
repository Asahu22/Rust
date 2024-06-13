use std::io;

fn main(){
    let mut s=String::new();
    io::stdin().read_line(&mut s).expect("Failed to read line");
    s=s.trim().to_string();
    let mut flag =true;
    let n=s.len();
    for i in 0..((n/2)-1){
        if s.as_bytes()[i]!=s.as_bytes()[n-1-i]{
            flag=false;
            break;
        }
    }
    if flag {
        println!("String is palindrome");
    } else {
        println!("String is not palindrome");
    }

}