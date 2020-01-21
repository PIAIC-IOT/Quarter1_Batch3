fn main() {
    let data1 =  "hello ";
    let data2 =  "Dobrý";
    let data3 =  "السلام عليكم";
    println!("Length of {} is {}",data1,data1.len());
    println!("Length of {} is {}",data2,data2.len());
    println!("Length of data3 is {}",data3.len());

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);


let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

//let s = s1 + "-" + &s2 + "-" + &s3;
let s =format!("{} -  {}  - {}",s1,s2,s3);

println!("{}",s);
println!("{}",s1);

let data = "PIAIC Batch3".to_string();
//          0123456789
let temp = &data[0..1];
println!("{}",temp);

}
