// enum Option<T>{
//     Some(T),
//     None
// }

fn main() {
    //let value2 = Option::Some(5); //user defined 
    let value1 = Some(5); //PreLude
    println!("{:?}",value1);
    let name1 = Some(String::from("Abdul Rehman"));
    println!("{:?}",name1);
    //let value10 = Option::None;  //user defince
    let nodata:Option<i32>=None; //PreLude
    
    println!("Next is from Match");
    
    match value1 {
        Some(a) =>  println!("{}",a+1),
        None    =>  println!("None"),
    }

    let number = match value1{
        Some(a) => a+1,
        None => 0
    };



    let coa = String::from("Qamar Javeed Bajwa");
    println!("name length of {:?} is {}",coa,coa.len());

    let age = [22,33,44,55];
    //index    0  1  2  3
    //let temp = 100;
    let data = age.get(2);
    println!("{:?}",data);


}
