fn main() {
    let name = String::from("Umair");
    let name_length = name.len();
    println!("{}",name_length);
    let age = [20,22,24,26];
    //element  0  1  2  3 4 5 6 7 8 9
    let temp = 9;
    println!("Age is : {}",age[temp]);
    //age.9 //tuple calling
    //age[3] //array calling
    //                     age[9]
    //println!("Age is : {:#?}",age.get(2));
    

}
