fn main() {
    let out = 5;

    let val = {
        let iner = 3;
        println!("The inner value is: {}", iner);
        println!("The out value is: {}", out);
        iner + 1
        
    };
    println!("The value is: {}", val);
}
