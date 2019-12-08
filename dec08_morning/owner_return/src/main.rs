fn main() {
    let get = giveback();
    println!("we got in get {}",get); 
    let batch3=String::from("IoT");
    let assignment=receiver(batch3);
    println!("we got in assignment {:?}",assignment); 
    println!("the length of {}, is {}",assignment.0,assignment.1); 
    let (zero,one)=receiver(get);
    println!("the length of {}, is {}",zero,one); 

}
fn giveback()->String{
    let name = String::from("Batch3");
    name
}

fn receiver(morning:String)->(String,usize){
    println!("We are in receiver Function");
    println!("{}",morning);
    let length = morning.len();
    (morning,length)
}