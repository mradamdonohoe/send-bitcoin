//bookmark vid 2

use std::io;
use rand::Rng;

fn send_bitcoin() {
    println!("we shall send some bitcoin");
    println!("send to who?");
    let clients = vec!["homer", "marge", "bart", "lisa"];
    
    for client in &clients {
        print!("{}", client);
    }

    let mut recipient = String::new();
    io::stdin().read_line(&mut recipient); 

    if clients.contains(&recipient.trim()) {
        println!("how much bitcoin?");

        let mut amount = String::new();
        io::stdin().read_line(&mut amount);

        println!("you sent {} bitcoin to {}", amount.trim(), recipient);
    } else {
        println!("person is not in your contacts");
    }
}

fn receive_bitcoin() {
    println!("we shall receive some bitcoint");

    let amount = rand::thread_rng().gen_range(1, 10);
    println!("you just received {} bitcoin", amount);
}

fn exit_console() {
    println!("Invalid option must be r or s");
}

fn console(){
    println!("\nlets have fun with bitcoin\n");
    println!("\ndo you want to send (s) or receive (r) bitcoin\n");

    let mut command = String::new();
    
    io::stdin().read_line(&mut command); 

    if command.trim().eq("s") {
        send_bitcoin();
    }  else if command.trim().eq("r") {
        receive_bitcoin();
    } else {
        exit_console();
    }
}

fn main() {
    console()
}
