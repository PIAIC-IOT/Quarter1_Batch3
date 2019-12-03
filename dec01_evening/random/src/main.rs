use rand::Rng;
fn main() {
    println!("Random Number Generator");
    let random_number = rand::thread_rng().gen_range(-100, 1001);
    println!("Random Number is : {}",random_number);
}
