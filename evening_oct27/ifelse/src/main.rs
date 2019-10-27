fn main() {
    println!("Welcome in main");
    let marks = 55;

    if marks > 50 {
        println!("Excellent marks");
    } else if marks > 20 {
        println!("Good marks");
    } else {
        println!("Need Improvement");
    }


    println!("Logical Operators");
    // And Operator
    if marks < 50 || marks > 20 {
        println!("Good marks");
    } else if marks > 50   {
        println!("Excellent marks");        
    } else {
        println!("Need Improvement");        

    }

    println!("End of program");

}
