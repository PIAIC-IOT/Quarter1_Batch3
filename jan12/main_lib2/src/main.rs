use add2n;
mod lib;
use std::io;  //standard libray io module

fn main() {
    loop {
    println!("Enter number 1");
    
    let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

    //let guess: u32 =  guess.trim().parse().unwrap();
         let guess: u32 = match guess.trim().parse() {
             Ok(num) => num,
             Err(_) => continue,
        };
        println!("Enter number 2");
        let mut guess2 = String::new();

        io::stdin().read_line(&mut guess2)
            .expect("Failed to read line");

    //let guess: u32 =  guess.trim().parse().unwrap();
         let guess2: u32 = match guess2.trim().parse() {
             Ok(num) => num,
             Err(_) => continue,
        };
        println!("You entered: {} and {} ", guess,guess2);
        //lib::table(guess);       
        //mytable::table2(guess);
        lib::add(guess,guess2);  
        add2n::add2(guess,guess2); 
        break;

    }  

}

// fn add(one:u32,two:u32){
//     println!("{} + {} = {}",one,two,one+two);
// }