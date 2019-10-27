fn main() {  //main scope
    let score = 10;
    println!("main score : {}",score);
    {  //child1 start
        println!("main in child score : {}",score);
        let child1 = "Child 1";
        println!("child 1 data : {}",child1);
    } //child1 end

    {  //child2 start
        println!("main in child score : {}",score);
        let child2 = "Child 2";
        println!("child 2 data : {}",child2);
    } //child2 end

    println!("End of program");
    println!("child 1 data : {}",child1);
    
}   //main scope
