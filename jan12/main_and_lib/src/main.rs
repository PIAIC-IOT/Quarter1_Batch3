use mytable; //cargo.toml mytable
mod lib; //same environment lib.rs

use std::io;  //standard libray io module

fn main() {
    loop {
    println!("Enter number to print table");
    
    let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

    //let guess: u32 =  guess.trim().parse().unwrap();
         let guess: u32 = match guess.trim().parse() {
             Ok(num) => num,
             Err(_) => continue,
        };
        println!("You entered: {}", guess);
        lib::table(guess);       
        mytable::table2(guess);
        break;

    }  
    
}

// fn table(data:u32){
//     for count in 1..11 {
//         println!("{} * {} = {}",data,count,data*count);
//     }
// }