#[derive(Debug)]
struct Student {
    //key:data type
    name:String,
    sub1:i32,
    sub2:i32
}
#[derive(Debug)]
struct Color(i32,i32,i32);

fn main() {
    let col1=Color(32,32,32);
    println!("Color  {:?}",col1.0);

    let name = String::from("Farjad");
    let sub1 = 88;
    let sub2 = 77;

    let ret_student=one(name,sub1,sub2);
    println!("returned  {:?}",ret_student);

    let stud_tuple = (String::from("Zaheer"),91,81);
    //                         0              1  2
    
    println!("Tuple name: {}",stud_tuple.0);
    println!("tuple  {:?}",stud_tuple);
    two(stud_tuple);    
    let student1 = Student {
        //key:value,
        name:String::from("1st Noman AIC"),
        sub1:95,
        sub2:85
    };
    println!("Struct  {:#?}",student1);
    println!("{}",three(student1));
}
// fn one (name:String,sub1:i32,sub2:i32) -> Student{
//     println!("we are in one function");
//     println!("{} {} {}",name,sub1,sub2);
//     let student3 = Student {
//         name,
//         sub1,
//         sub2
//     };
//     println!("{:?}",student3);
//     student3
// }

fn one (name:String,sub1:i32,sub2:i32) -> Student{
    Student {
        name,
        sub1,
        sub2
    }
}
fn two(st_tuple:(String,i32,i32)){
    println!("{:?}",st_tuple);
}

fn three(st_struct:Student) -> i32{
    println!("We are in function 3");
    println!("{:#?}",st_struct);
    st_struct.sub1+st_struct.sub2
}