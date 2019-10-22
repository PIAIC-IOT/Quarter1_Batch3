fn main() {
    let data = ("PIAIC", 3000, 67.8);
    let (org, fee, percent) = data;
    println!("The fee is: {}", fee);
    let org1 = data.0;
    println!("The Organization is: {}", org1);
    println!("Complete Tuple {:?}",data);

    let mut fruit = ["Mango","Orange","Banana","Papaya","Cheeko"];
    println!("Fruit array {:#?}",fruit);
    fruit[0] = "Sindhri Mango";
    println!("Fruit array {}",fruit[0]);
    let temp:usize = 2;
    println!("Fruit.2 array {}",fruit[temp]);

}