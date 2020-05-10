#[derive(Debug)]
struct Food {
    name : String,
    price : u16,
    type1 : String,
}

fn main() {
    let mut food1 = ("Haleem".to_string(),100,"Desi".to_string());
    //index        0                   1    2
    println!("food name : {:?}",food1.0);
    food1.0 = "Chicken Haleem".to_string();

    println!("food name : {:?}",food1.0);
    let food2 = Food {
        name : "Paya".to_string(),
        type1 : "Desi".to_string(),
        price : 250,
    };

    let mut food3:Food;
    food3 = Food {
        name : "Khaya".to_string(),
        type1 : "Desi".to_string(),
        price : 250,
    };
    println!("{:?}  ",food3);
    println!("{:?}  ",food2.name);
}
