use rand::Rng;
fn main() {
      let number = rand::thread_rng().gen_range(1.0, 51.0);

    println!("The secret number is: {}", number);
    let number1 = rand::thread_rng().gen_range(1.0, 51.0);
    println!("The secret number is: {}", number1);
}
