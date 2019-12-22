#[derive(Debug)]
struct Student {
    name:String,
    score:u8,
}
#[derive(Debug)]
struct Teacher {
    name: String,
    salary: u8
}
struct Score {
    score1:u8,
    score2:u8,
}

impl Teacher {
    fn print_salary(&self) {
        println!("{:?}", self)
    }
}


impl Student {
    fn impl_print(&self){
        println!("{:?}",self.name);
    }

}
fn print(student1:& mut Student){
    println!("{:?}",student1.name);
}

fn main() {
    let mut student1 = Student {
        name:"Waleed".to_string(),
        score:86,
    };
    
    let guest= Student{
        name:"Taimoor Imtiaz".to_string(),
        score:100,
    };
    print(&mut student1); //simple function calling
    guest.impl_print(); //method function calling    
    println!("in main {:?}",guest);


    let teacher1 = Teacher {
        name: "Imran".to_string(),
        salary: 120
    };

    teacher1.print_salary();
}

