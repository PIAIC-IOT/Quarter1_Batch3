fn main() {
    let mut age:u16 = 0;
    loop {
        if age >5 {
            break;
        }
        println!("Loop : Age is : {}",age);
        age = age + 1;    
        
    }

    let mut score:u16 = 0;

    while score <=5 {
        println!("While :  score is : {} ",score );
        score +=1;
    }

    //for owais in 0..<5 {
       for owais in (0..=5).rev() {
        println!("for :  value is : {}",owais);
    }

    // println!("Welcome Batch 4");
    // println!("Welcome Batch 4");
    // println!("Welcome Batch 4");
    // println!("Welcome Batch 4");
    // println!("Welcome Batch 4");
    // println!("Welcome Batch 4");
    // println!("Welcome Batch 4");
    // println!("Welcome Batch 4");
    // println!("Welcome Batch 4");
    // println!("Welcome Batch 4");
}
