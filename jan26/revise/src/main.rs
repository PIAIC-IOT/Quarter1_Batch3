#[derive(Debug)]
enum Games {
    pubg,
    fortnight,
    cs4,
}
impl Games {
    fn ppt(&self) {
        println!("in ppt {:?}",self);
    }
    fn second(receive: u8) -> Games {
        println!("in second {}",receive);
        match receive {
            1 => Games::pubg,
            2 => Games::fortnight,
            _ => Games::cs4,
        }
    }
}
fn third(abc:u8){
    println!("in third {}",abc);
}
fn main() {
    let game1 = Games::pubg;
    println!("{:?}",game1);
    game1.ppt();
    let ret = Games::second(99);
    println!("{:?}",ret);
    third(88);
    Games::second(1).ppt();

    io::stdin.read_line(&mut abc).unwrap()

}



