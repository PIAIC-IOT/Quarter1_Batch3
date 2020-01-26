use std::io;

fn main() {
    println!("Enter any number");
    let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: usize =  guess.trim().parse().expect("Not a Number");
            
            // let guess: u32 = match guess.trim().parse() {
        //     Ok(num) => num,
        //     Err(_) => panic!("Too large"),
        // };

        println!("You guessed: {}", guess);

        if guess >5 {
            panic!("Too large");
        }

    let data = [11,22,33];
    let temp = &data[guess];
    //println!("{}",data[guess]);

}