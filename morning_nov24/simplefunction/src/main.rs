fn main() {
    
    println!("\n First Calling");
    let receive = printer(5);
    println!("{}",receive);
    let receive = printer(60);
    println!("{}",receive);
    // printer();
    // println!("\n Third Calling");
    // printer();
    // println!("\n Fourth Calling");
    // printer();
}

fn printer(pass:i32) -> i32{
    println!("Editing / Updating");
    pass+50
}

