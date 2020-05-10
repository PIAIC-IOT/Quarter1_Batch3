use string_length;
//use std::io;

fn main() {
    let name = "Haseeb".to_string();
    let result = string_length::mystring::len_calculator(name);
    println!("Returned length is {}",result);

}
