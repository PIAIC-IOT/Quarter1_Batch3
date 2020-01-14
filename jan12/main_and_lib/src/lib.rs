pub fn table(data:u32){
    println!("Next line is from lib.rs");
    for count in 1..3 {
        println!("{} * {} = {}",data,count,data*count);
    }
}