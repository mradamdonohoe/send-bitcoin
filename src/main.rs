//bookmark vid 1 10:48

use std::io;

fn console(){
    println!("\nlets have fun with bitcoin\n");
    println!("\ndo you want to send (s) or receive (r) bitcoin\n");

    let mut command = String::new();
    
    io::stdin().read_line(&mut command);    
}

fn main() {
    console()
}
