use rand::Rng;
fn main() {
println!("Guess the number!");
let secret_number =rand::thread_rng().gen_range(1, 101);
println!("The secret number is: {}", secret_number);
}
