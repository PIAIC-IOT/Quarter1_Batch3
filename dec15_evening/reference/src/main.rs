fn main() {
    let name = String::from("Batch3");
    let name1 =&name;
    println!("{}",name);
    println!("{}",name1);
    one(&name);
    let mut course = String::from("IoT ");
    let course1 =& mut course;
    println!("{}",course1);
    course1.push_str(" Evening");
    println!("{}",course1);
    println!("{}",course);
    let mut age:u8 = 55;
    let age2=&mut age;
    println!("{}",age2);
}

fn one(s:&String){
    println!("we are in one {}",s);
}
