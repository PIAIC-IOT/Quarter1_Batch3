fn main() {
    let cup1:u8 = 100 ;
    println!("cup1 : {} ml",cup1);
    let cup2 = cup1;
    println!("cup1 : {} ml",cup1);
    println!("cup2 : {} ml",cup2);

    let food1 = String::from("Biryani");
    println!("food1 : {} ",food1);
    let food2 = food1;
    println!("food1 : {} ",food1);
    println!("food2 : {} ",food2);
}
