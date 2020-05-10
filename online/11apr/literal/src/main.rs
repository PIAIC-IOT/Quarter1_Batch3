//add following in cargo.toml
// [dependencies]
// rand = "0.6.0"

use std::io;
use rand::Rng;
//#include(stdio.h)

fn main() {
    let mut name_lit = "Haseeb";  //string literal
    
    let mut name_str = "Hassan".to_string(); //string type
    println!("name_lit {}",name_lit);
    println!("name_str {}",name_str);
    name_str.push_str(" Ali");
    //name_ltr.push_str(" Ali");
    name_str = name_str + " Ali";
    println!("name_lit {}",name_lit);
    println!("name_str {}",name_str);
    let mut age = 22;
    age = age +2 ;

    let mut user_input = String::new();  //user_input is a variable name
    //let mut user_input = "";  //user_input is a variable name
    println!("Type Your Name:");
    io::stdin().read_line(&mut user_input).expect("Failed to read line");
    //io::stdin().read_line(&mut user_input).unwrap();
    println!("You entered : {:#?}",user_input);
    println!("You entered : {:#?}",user_input.trim());
    
    
    let secret_number:u8 = rand::thread_rng().gen_range(1, 101);
    println!("Lottery Number : {:#?}",secret_number);
    let dice:f32 = rand::thread_rng().gen_range(1.1, 7.7);
    println!("Lottery Number : {}",dice);
    

}
