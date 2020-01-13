enum Option<T> {
    Some(T),
    None
}

enum Result<T,E> {
    Ok(T),
    Err(E)
}

#[derive(Debug)]
struct Weeks {
    day1:String,
    day2:String,
}
#[derive(Debug)]
enum Days {
    Sunday(String),
    Monday(u8,u8),
    Tuesday,
}

#[derive(Debug)]
enum Vehicle {
    Toyota(String,u16),
    Honda(String,u16),
    Chingchi
}
impl Days {
    fn one(&self){
        println!("{:?}",self);
    }
}
fn two (data:Days){
    println!("we are in two function");
    match data {
        Days::Sunday(a)  => println!("{}",a),
        Days::Monday(x,y) => println!("{} {}",x,y),
        Days::Tuesday =>    println!("No Task"),     
    }
}
fn main() {
    
    
    let week1 = Weeks {
        day1:String::from("Friday"),
        day2:String::from("Saturday"),
    };

    let day1 = Days::Sunday(String::from("Holiday"));
    let day2 = Days::Monday(222,33);

    println!("{:#?}",week1);
    //println!("{:#?}",day1);
    //println!("{:#?}",day2);
    day1.one();
    day2.one();
    println!("Next result is from match of an enum");
    match day1 {
        Days::Sunday(a)  => println!("{}",a),
        Days::Monday(x,y) => println!("{} {}",x,y),
        Days::Tuesday =>    println!("No Task"),
    }

    two(day2);

    let car1 = Vehicle::Toyota("Corolla".to_string(),2019);
    println!("{:#?}",car1);

    let num1 = 2u8;
    let num2:u8 = 2;
    println!("Next Lines are from match");
    match num1 {
        0 => println!("Welcome 0"),
        1 => println!("1"),
        2 => {println!("Welcome");
              println!("Welcome 2");  },
        3 => println!("3"),
        _ => println!("Out of range"),
    }
}






