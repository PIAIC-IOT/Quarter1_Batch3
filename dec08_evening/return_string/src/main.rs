
fn main() {
    let p1 = cricket();
    println!("p1 is {}",p1);

    let football_player=String::from("Ronaldo");
    
    let p3= football(football_player);
    println!("p3 in main {:?}",p3);
    println!("p3 in main {} {} ",p3.0,p3.1);

    let team3=String::from("Islamabad United");
    let (t1,len)= football(team3);
    println!("t1  {} len {} ",t1,len);

}

fn cricket() -> String{
     let player1=String::from("Babar Azam");
     player1
    //String::from("Babar Azam")
}

fn football(p2:String)-> (String,usize) {
    println!("We are in football function");
    println!("{}",p2);
    let length = p2.len();
    (p2,length)

}
