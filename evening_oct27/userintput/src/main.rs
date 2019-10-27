use std::io;
//std/io/

fn main() {
    println!("Enter any data");
    let mut inputdata = String::new();
    io::stdin().read_line(&mut inputdata).expect("Failed to read line");
    println!("You entered        : {}",inputdata);
    println!("Pretty You entered : {:?}",inputdata);
    println!("Trim You entered : {:?}",inputdata.trim());
    let int_data: u32 = inputdata.trim().parse().expect("Please type a number!");
    println!("Converted Integer : {:?}",int_data);    
}
