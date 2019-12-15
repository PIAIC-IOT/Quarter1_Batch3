#[derive(Debug)]
struct Student {
    //key:data type
    name:String,
    sub1:i32,
    sub2:i32
}

#[derive(Debug)]
struct Food {
    //key:data type
    price:i32,
    quantity:i32
}

fn main() {
    let name = String::from("Farjad");
    let sub1 = 88;
    let sub2 = 77;

    one(name,sub1,sub2);

    let stud_tuple = (String::from("Zaheer"),91,81);
    //                         0              1  2
    println!("Tuple name: {}",stud_tuple.0);
    println!("tuple  {:?}",stud_tuple);
    let student1 = Student {
        //key:value,
        name:String::from("1st Noman AIC"),
        sub1:95,
        sub2:85
    };

    println!("Struct name: {}",student1.name);
    println!("sub1 name: {}",student1.sub1);
    println!("sub2 name: {}",student1.sub2);

    let student2 = Student {
        //key:value,
        sub2:55,
        name:String::from("2nd Mehak"),
        sub1:40
        
    };
    println!("Struct name: {}",student2.name);

    let mut student3 = Student {
        name:String::from("Major Tariq"),
        sub1:88,
        sub2:66
    };

    println!("Struct name: {}",student3.name);
    println!("Struct sub2: {}",student3.sub2);

    student3.sub2=77;
    println!("Struct sub2: {}",student3.sub2);
    println!("Struct  {:#?}",student3);

    let mut student4 = Student {
        name:String::from("Waleed"),
        sub1:student3.sub1,
        sub2:66
    };    
    println!("Struct  {:#?}",student4);
    
    let mut student5 = Student {
        name:String::from("Zahid"),
        ..student4
    };    
    println!("Struct  {:#?}",student5);

    let student6=student2;
    println!("Struct  {:#?}",student6);
    //println!("Struct  {:#?}",student2);

    let food1 = Food {
        price:22,
        quantity:4
    };

    println!("Struct  {:#?}",food1);

    let food2=food1;
    println!("Struct  {:#?}",food2);
    //println!("Struct  {:#?}",food1);
    

}
fn one (name:String,sub1:i32,sub2:i32){
    println!("we are in one function");
    println!("{} {} {}",name,sub1,sub2);
}