mod lib;
use lib::Diceroll;
use std::io;
use rand::Rng;
fn main () {
    println!("Dice Game!");
    loop {
    let dice = rand::thread_rng().gen_range(1, 7);
    
    let myroll = match dice {
            1 => Diceroll::Mydice::One,
            2 => Diceroll::Mydice::Two,
            3 => Diceroll::Mydice::Three,
            4 => Diceroll::Mydice::Four("Chocolate".to_string()),
            5 => Diceroll::Mydice::Five,
            6 => Diceroll::Mydice::Six("Chocolate".to_string()),
            _ => Diceroll::Mydice::Zero,
        };
        Diceroll::ppt(&myroll);
        println!("Do you want to continue y/n ?");
        let mut cont = String::new();
        io::stdin().read_line(&mut cont).expect("Failed to read line");
    
        if cont.trim() == "n".to_string() || cont.trim() == "N".to_string(){
            break;
        }
    }
    
}
    // loop {
    //     println!("Please input your guess.");
    //     let mut guess = String::new();

    //     io::stdin().read_line(&mut guess)
    //         .expect("Failed to read line");

    //     let guess: u32 = match guess.trim().parse() {
    //         Ok(num) => num,
    //         Err(_) => continue,
    //     };

    //     println!("You guessed: {}", guess);
    //     break;
    
    // }