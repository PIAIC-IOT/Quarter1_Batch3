#[derive(Debug)]
struct Food {
    name:String,
    price:u16,
    serving:u8,
}
impl Food {
    fn afaq(){
        println!("We are in afaq Associated FUnction");
        println!("December 22, 2019");
    }
    fn newinstance(name:String,price:u16,serving:u8) -> Food{
        Food{
            name,
            price,
            serving
        }
    }    //end of first
  fn welcome(){
        println!("We are in welcome Associated Function");
        println!("We welcome Mr. Taimoor Imtiaz");
        println!("Thank you so much for your time");
    } //end of welcome
} //end of first implementation

// impl Food {
  
// }

fn newinstance(name:String,price:u16,serving:u8) -> Food{
    Food{
        name:name,
        price:price,
        serving:serving
    }
}

fn afaq(){
    println!("We are in afaq Simple FUnction");
    println!("December 22, 2019");
}

fn main() {
    let name = String::from("Omlete + Bread");
    let price:u16 = 200;
    let serving:u8 = 1;
    
    
    let name2 = String::from("Omlete + Paratha");
    let price2:u16 = 250;
    let serving2:u8 = 1;
    
    let nashta = newinstance(name,price,serving);
    println!("{:#?}",nashta);
    let nashta2 = Food::newinstance(name2,price2,serving2);
    println!("{:#?}",nashta2);
    Food::welcome();
    afaq(); //simple function
    Food::afaq(); //simple function
    //team1.print(); // method calling
    

}
