fn main() {
    quiz();
}

fn quiz(){
    let mut loopdata=0;
    for data in 0..20 {
        println!("Value of data is {}",data);
        loopdata=data;
    }
    println!("Value of data is {}",loopdata);
}