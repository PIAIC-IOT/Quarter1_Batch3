#[derive(Debug)]
struct Data {
    name:String,
    fee:u16,
    status:bool,
}
#[derive(Debug)]
enum Pakistan {
    Punjab,
    Sindh,
}

fn main() {
    let student1 = Data {
        name:String::from("Rehman "),
        status:true,
        fee:3000
    };
    println!("{:#?}",student1);
    let p1 = Pakistan::Punjab;
    println!("{:#?}",p1);
    
}
