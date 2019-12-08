fn main() {
    let team1=String::from("Islamabad United");
    println!("Team1 is {}",team1);
    println!("Pointer is {:p}",&team1);
    //let team2=team1;
    //println!("Team1 is {}",team1);
    let team3=&team1;
    println!("Team1 is {}",team1);
    println!("Team3 is {}",team3);

    let len = t20(&team1);
    println!("Team1 length is {}",len);

}

fn t20(team5:&String)-> usize {
    let length = team5.len();
    length
}
