#[derive(Debug)]
enum Direction {
    Right,
    Left,
    Up(String),
    Down(u8)
}

impl Direction {
    fn impl_ppt(&self) {
//              &Direction        Readable
//              Direction         Move
//              &mut Direction    writeable
        println!("We are in impl_ppt Method");
        println!("{:?}",self);
    }

    fn impl_new(faisal:u8)->Direction{
        Direction::Down(faisal)
    }

}

//fn ppt(age:u32,name:String,salary:u32) {

fn ppt(data:Direction) {
    println!("We are in ppt Function");
    println!("{:?}",data);
}

fn new1(faisal:u8)->Direction{
    Direction::Down(faisal)
}

fn main() {
    let age : u8 = 55;  //enum
    let age_data = [22,24,26,28]; //struct    
    let d1 = Direction::Right;
    println!("{:?}",d1);
    let d2 = Direction::Left;
    ppt(d2);
    let d3 = Direction::Up(String::from("Drive Slow")); //string Type
    d3.impl_ppt();
    println!("from main fn : {:?}",d3);
    let d4 = new1(40);
    d4.impl_ppt();

    Direction::impl_new(20).impl_ppt();
    //String::from();
    //let d3 = Direction::Up("Drive Slow".to_string()); //string Type
    //let d3 = Direction::Up("Drive Slow"); //string literal
    //let mut name = String::from("Hello");

}








