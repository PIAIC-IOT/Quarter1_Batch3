fn main() {
    let mut data  = ("PIAIC", 3000, 67.8);
    let (org, fee, percent) = data;
    println!("The fee is: {}", fee);
    let org1 = data.0;
    println!("The Organization is: {}", org1);

    println!("Complete Tuple: {:?}", data);
    
    data.1 = 2000;

    println!("Update   Tuple: {:?}", data);
}

