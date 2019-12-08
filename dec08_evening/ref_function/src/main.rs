fn main() {
    let mut meal1=String::from("Kabuli Plao");
    food(&mut meal1);
    println!("in main : {}",meal1);
    
}

fn food(m1:&mut String){
    println!("We are in food function");
    println!("{}",m1);
    m1.push_str(" + Kofta");
    println!("{}",m1);

}
