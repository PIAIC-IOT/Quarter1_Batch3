fn main() {
    {
        let age:u8 = 55;
        println!("Age is : {}",age);
        println!("Pointer of Age is : {:p}",&age);
    }
    
    
    let mut student_name=String::from("MUHAMMAD YASIR RAJPUT");
    println!("Name  : {}",student_name);
    println!("Length  : {}",student_name.len());
    println!("Pointer of name is : {:p}",&student_name);
    student_name.push_str(" Janjua");
    println!("Updated Name  : {}",student_name);
    println!("Length  : {}",student_name.len());
    
    let data = 123.to_string(); //string Type
    student_name.push_str(&data);
    println!("Updated Name  : {}",student_name);    
    let parter = "HASSAN";  //string literal
    let datastring= parter.to_string(); //string Type
    println!("data: {}, datastring {}",data,datastring);

}
