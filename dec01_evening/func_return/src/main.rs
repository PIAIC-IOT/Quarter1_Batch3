fn main() {
    println!("Start of main");
    let num2 = 4;
    println!("{:?}",sum(5,num2));
    //let abc =sum(5,num2);
    //abc.0
    //abc.1
    //sum();
    println!("direct {}",sum2());
    let rec=sum2();
    println!("return by variable{}",rec);
    println!("End of main");
}

fn sum (val1:u32,val2:u32) -> (u32,u32) {
    println!("we are in sum function");
    println!("{}",val1+val2);
    (val1,val1+val2)
}

fn sum2()->u8{
    println!("we are in sum2 function");
    let age =4;
    age
}