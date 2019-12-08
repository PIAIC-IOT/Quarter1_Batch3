fn main() {
    let age:u8=55;
    let age1=age;
    let age2=&age;
    let player1=String::from("Jaweed MianDad");
    println!("player1 : {}",player1);
    println!("player1 pointer : {:p}",&player1);
    let player2=&player1;
    println!("player1 : {}",player1);
    println!("player2 : {}",player2);
    let player3=&player1;
    let player4=&player1;

    let mut team3=String::from("Manchester");
    println!("team3 : {}",team3);
    let team4=&mut team3;
    let team5=&mut team3;
    println!("team4 : {}",team4);
    team4.push_str(" United");
    println!("team4 : {}",team4);
    println!("team3 : {}",team3);

}
