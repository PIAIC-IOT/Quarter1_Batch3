#[derive(Debug,Clone)]
struct Student {
    name:String,
    subject1:i32,
    subject2:i32,
}

fn main() {
    let name = String::from ("IoT");
    let subject1 = 32;
    let subject2 = 50;
    let st1= one(name,subject1,subject2);
    println!("{:#?}",st1);
    println!("{}",two(&st1));
    let std2=st1.clone();

}

fn one(name:String,subject1:i32,subject2:i32) -> Student{
    //println!("{} {} {}",n1,s1,s2);
    Student {
        name,
        subject1,
        subject2,
    }
}

fn two (s:&Student)->i32 {
    s.subject1+s.subject2
}

