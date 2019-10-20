fn main() {
    let value1 = 22;
    let value2 = 6;

    let plus = value1 + value2;
    let minus = 22-2;
    let div = value1/value2;
    let mul = value1*value2;
    println!("Plus:  {} + {} = {}",value1,value2,plus);
    println!("Minus: {} - {} = {}",value1,value2,minus);
    println!("Div:   {} / {} = {}",value1,value2,div);
    println!("Mul:   {} * {} = {}",value1,value2,mul);
    println!(":");
    let modulus = value1%value2;
    println!("Modulus:   {} % {} = {}",value1,value2,modulus);
    let name = "PIAIC Batch 3";
    println!("String Literal:   {} ",name);
    let grade = 'A';
    let enroll = true;
    println!("Grade:   {} Enrollment status : {}",grade,enroll);

    let val1 =55;
    let val2 = 45.6;
    let sum = val1 as f64+ val2 ;
    println!("{} ",sum);
}







