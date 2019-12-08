fn main() {
    let mut football_player=String::from("Christiano ");
    println!("Player {}",football_player);
    //football_player.push_str("Ronaldo");
    //println!("{}",football_player);
    let football_player1=& mut football_player;
    println!("Player1 {}",football_player1);
    football_player1.push_str("Ronaldo");
    println!("Player1 {}",football_player1);
    println!("Player {}",football_player);
    let mut team2=  String::from("Lyari ");
    println!("team2 {}",team2);
    football(&mut team2);
    println!("team2 {}",team2);

    let mut p5=String::from("Batch3");
    let p6 = &mut p5;
    println!("Player {}",p6);
    let p7 = &mut p5;
    let p8 = & p5;
    let p9 = & p5;
    let p10 = & p5;


}

fn football(p2:&mut String){
    p2.push_str(" Karachi")
}
//Christiano
//Christiano + Christiano Ronaldo
//Compile time error
//Panic