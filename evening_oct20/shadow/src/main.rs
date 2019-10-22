fn main() {
    let mut age = 33 ;
    println!("Age is : {}",age);
    println!("Pointer of Age is : {:p}",&age);
    age = 34 ;
    println!("Now Age is : {}",age);
    println!("Pointer of Age is : {:p}",&age);
    //age = "PIAIC";

    let salary = 43_566;
    println!("Salary is : {}",salary);
    println!("Pointer of Salary is : {:p}",&salary);
    let salary = salary + 5_000;
    println!("Salary is : {}",salary);
    println!("Pointer of Salary is : {:p}",&salary);
    let salary = "Forty Eight Thousand";
    println!("Salary is : {}",salary);
    println!("Pointer of Salary is : {:p}",&salary);
}
