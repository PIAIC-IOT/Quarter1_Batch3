
fn main() {
    let mut food1=String::from("Paratha");
    let food2= &mut food1;
    println!("{} ",food2);
    let mut food3=String::from("Nan ");
    let food4=&mut food3;
    println!("{} ",food4);
    food4.push_str(" Qatai");
    println!("{} ",food4);
    println!("{} ",food3);



}
