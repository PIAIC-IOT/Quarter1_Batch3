use std::io;

fn main() {

    let mut user_input = String::new();  //user_input is a variable name
    //let mut user_input = "";  //user_input is a variable name
    println!("Type Your Name:");
    io::stdin().read_line(&mut user_input).expect("Failed to read line");
    //io::stdin().read_line(&mut user_input).unwrap();
    println!("You entered : {:#?}",user_input);
    println!("You entered : {:#?}",user_input.trim());
    
    
    let mut height = String::new();  
    //let mut user_input = "";  //user_input is a variable name
    
    println!("Type Your Height:");
    io::stdin().read_line(&mut height).expect("Failed to read line");
    //io::stdin().read_line(&mut user_input).unwrap();
    println!("You entered : {:#?}",height);
    println!("You entered : {:#?}",height.trim());
    let height: f32 = height.trim().parse().unwrap();
    println!("You entered : {:#?}",height);
    loop {
        let mut temp = String::new();  
    //let mut user_input = "";  //user_input is a variable name
    
    println!("Type today Temperature:");
    io::stdin().read_line(&mut temp).expect("Failed to read line");
    //io::stdin().read_line(&mut user_input).unwrap();
    println!("You entered : {:#?}",temp);
    println!("You entered : {:#?}",temp.trim());
    
    let temp: f32 = match temp.trim().parse() {
        Ok(data) => data,
        Err(_)   => continue,
    };
    println!("You entered : {:#?}",temp);
    break;

    }
     
    let val:u8 = 66;
    let age :u8 = 66;
    match age {
        0...50 => println!("You are young"),
        val    => println!("You are {} year old",val),
        _      => println!("out of range"),
    };
}
