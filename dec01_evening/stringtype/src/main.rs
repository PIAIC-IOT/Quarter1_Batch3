fn main() {
    {
        let age = 55;
    }
    println!("age {}",age);
    
    {  //scope of string
    let name1 = String::from("PIAIC");
    println!("name1 {}",name1);
    let name2 = String::new();
    let name3 = "IOT".to_string();
    let temp = " Batch 3";
    let mut name4 = temp.to_string();
    println!("name 4  {}",name4);
    name4.push_str(" Islamabad");
    println!("after push_str name 4  {}",name4);
    } //END scope of string
    //println!("after push_str name 4  {}",name4);

}
