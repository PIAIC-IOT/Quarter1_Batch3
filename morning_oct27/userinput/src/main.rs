use std::io;

fn main() {
    let mut input1 = String::new();
    println!("Enter your data");
    io::stdin().read_line(&mut input1).expect("Failed to read line");
    println!("You enter: {:?}",input1.trim());
    println!("You enter: {:?}",input1);
    let int_input: u32 = input1.trim().parse().expect("Please type a number!");
    println!("You enter: {:?}",int_input);
}
