pub fn table2(data:u32){
    println!("Next line is from mytable::table2");
    for count in 1..3 {
        println!("{} * {} = {}",data,count,data*count);
    }
}
