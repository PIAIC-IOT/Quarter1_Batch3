#[derive(Debug)]
enum Product{
    AC,
    Fan(String),
    Heater
}

impl Product {
    fn ppt(&self){
        println!("Printing from method {:?}",self);
    }

    fn sort(data:&Product){
        println!("Next lines are from sort associated function");
        match data {
            Product::AC => println!("Data of AC {:?}",data),
            Product::Fan(f) => println!("Data of Fan {:?}",f),
            Product::Heater => println!("Data of heater {:?}",data),
        }
    }
}
fn main() {
    //println!("Hello, world!");
    let product1=Product::AC;
    product1.ppt();
    //println!("Product 1 : {:?}",product1);
    let product2=Product::Fan("For Summer".to_string());
    product2.ppt();
    Product::sort(&product1);
    Product::sort(&product2);
    //println!("Product 2 : {:?}",product2);

    let arraydata = [11,22,33,44,55];
    //  index        0  1  2  3   4
    let temp = arraydata.get(4);
    println!("{:?}",temp);
    let name = "Abdul Rehman".to_string();
    println!("lenght of name {} is {}",name,name.len());

}

