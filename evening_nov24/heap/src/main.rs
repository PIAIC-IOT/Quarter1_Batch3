fn main() {
    let name1 = "PIAIC"; //string literal
    println!("String Literal name 1 : {}",name1);
    let name2 = name1;
    println!("String Literal name 1 : {}",name1);
    println!("String Literal name 2 : {}",name2);
    
    let batch1 = String::from("Batch 3"); //string TYPE
    println!("String Type batch1 : {}",batch1);
    let batch2 = batch1;
    //println!("String Type batch1 : {}",batch1);
    println!("String Type batch2 : {}",batch2);

    let age1 = 55;
    println!("Scalar Type age1 : {}",age1);
    let age2 = age1;
    println!("Scalar Type age1 : {}",age1);
    println!("Scalar Type age2 : {}",age2);

}
