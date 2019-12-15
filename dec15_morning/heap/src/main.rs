fn main() {
    let age = 55; //i32 primitive data type STACK
    println!("age {}",age);
    let age2 = age;
    println!("age {}",age);
    println!("age2 {}",age2);
    let player1= "Shahid Afridi"; //String Literal stack
    let player2=String::from("Faraz Ahmed"); //String Type heap
    println!("{}",player2);
    let player3 = player2;
    println!("{}",player3);
    abcd(player3);
    

} 

fn abcd (p:String){
    println!("we are in abcd {}",p);
}//drop player3