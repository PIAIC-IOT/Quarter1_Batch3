pub mod Diceroll {
#[derive(Debug)]
pub enum Mydice {
    Zero,
    One,
    Two,
    Three,
    Four(String),
    Five,
    Six(String)
}

pub fn ppt(roll:&Mydice){
    match roll {
        Mydice::One => println!("One Go : Hmm!"),
        Mydice::Two => println!("Two Go : keep it up"),
        Mydice::Three => println!("Three Go : good!"),
        Mydice::Four(extra) => println!("Lovely Four Go and a {} gift",extra),
        Mydice::Five => println!("Five Go : Good Job"),
        Mydice::Six(extra) => println!("High Sixer Go and a {} gift",extra),
        Mydice::Zero => println!("Zero"),
    };
}
}