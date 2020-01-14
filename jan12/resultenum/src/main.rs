// c:/abc/xyz/class/rehman.jpg
// crate::abc::xyz::class::rehamn();
// Product::sort
// Product/sort
use std::io;
fn main() {
    loop {
    println!("Guess the number!");
    
    let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

    //let guess: u32 =  guess.trim().parse().unwrap();
         let guess: u32 = match guess.trim().parse() {
             Ok(num) => num,
             Err(_) => continue,
        };
        break;

        println!("You guessed: {}", guess);
    }  
}
