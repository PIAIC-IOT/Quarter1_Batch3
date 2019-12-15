#[derive(Debug)]
struct Student {
    name:String,
    subject1:i32,
    subject2:i32,
}

#[derive(Debug)]
struct Food {
     quantity:u32,
     price:u32
}

fn main() {
    let name = String::from("Faisal");
    let sub1 = 80;
    let sub2 = 75;
    println!("{} {} {}",name,sub1,sub2);
    let student1 = (String::from("Abdul Rehman"),85,88);
                //0                          1  2
    println!("{:?}",student1);
    println!("{}",student1.0);

    let stud1 = Student {
        name:String::from("Talemand"),
        subject1:80,
        subject2:90,
    };

    println!("name: {}",stud1.name);
    println!("Subject 1 marks {}",stud1.subject1);
    println!("Subject 2 marks {}",stud1.subject2);
    println!("{:#?}",stud1);

    let mut stud2= Student {
        subject2:90,
        name:String::from("Muneeb"),
        subject1:97,
    };

    println!("{:#?}",stud2);
    stud2.subject2=99;
    println!("{:#?}",stud2);
    
    let stud3=stud2;
    println!("{:#?}",stud3);
    //println!("{:#?}",stud2);

    let stud4 = Student {
        name:String::from("Sana Ullah"),
        subject1:stud3.subject1,
        subject2:stud3.subject2,
    };

    println!("{:?}",stud4);
    println!("{:?}",stud3);

    let stud5 = Student {
        name:String::from("Mazhar"),
        ..stud4
    };

    println!("{:?}",stud5);

    let food1 = Food {
        quantity:1,
        price:100,
    };

    println!("{:?}",food1);

    let food2=food1.clone();
    //println!("{:?}",food1);

}
