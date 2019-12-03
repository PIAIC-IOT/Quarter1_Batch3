fn main()  {
    let number:u32 = 233;
    let receive = square(number);
    println!("{:?}",receive);
 }  
fn square(val: u32) -> (u32,u32)  {  //Function Signature  
    println!("{}",val);
    (val,val*val)

 }