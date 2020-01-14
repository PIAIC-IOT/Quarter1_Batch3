#[derive(Debug)]
enum Option {
    Some(u8),
    None
}

fn main() {
   let data1 = Option::Some(10);
   println!("{:?}",data1);
   //let data3:u8=20;
   //let result = data1+data3;
   //println!("{:?}",data1);
   let data2=Option::None;
   println!("{:?}",data2);
}   
