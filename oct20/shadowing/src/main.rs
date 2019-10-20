fn main() {
    let mut age = 22;
    println!("Value of Age is : {}",age);
    println!("    Pointer of Age is : {:p}",&age);
    age = 33;
    println!("Update Value of Age is : {}",age);
    println!("New Pointer of Age is : {:p}",&age);
    //age = "forty four";
    let salary = 45_000;
    println!("Salary is : {}",salary);
    println!("Pointer  is : {:p}",&salary);

    let salary = salary + 3_000;
    println!("Salary is : {}",salary);
    println!("Pointer  is : {:p}",&salary);
    let salary = "Three Thousand";
    println!("Salary is : {}",salary);
    println!("Pointer  is : {:p}",&salary);

    let salary = salary.len();
    println!("Length  is : {}",salary);
}
