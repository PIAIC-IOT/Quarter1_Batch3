mod hr {
    pub mod interview {
        #[derive(Debug)]
        pub struct Data {
         name:String,
            pub score:u8
        }
        #[derive(Debug)]
        pub enum car_Type{
            Automatic(String),
            Manual(String)
        }
        pub fn int () {
            println!("We are in HR::interview::int");
        }
    }

    pub mod attendance {
        pub fn time(){
            println!("We are in HR::attendance::time");
        }

    }
}

use crate::hr::interview;
use crate::hr::interview::int; //idiomatic path

fn main() {
    interview::int(); // Absolute path
    int();  //idiomatic path
    hr::attendance::time(); // Relative path

    let data1 = hr::interview::Data {
        name:"Rehman".to_string(),
        score:33
    };
    println!("{:?}",data1);
}
