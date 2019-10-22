fn main() {
   let value1  = 56;
   let value2 = 33;

   println!("Sum: {} + {} = {} ", value1, value2, value1 + value2);
   println!("Sub: {} - {} = {} ", value1, value2, value1 - value2);
   println!("Mul: {} * {} = {} ", value1, value2, value1 * value2);
   println!("Div: {} / {} = {} ", value1, value2, value1 / value2);

   let modulus = value1 % value2;
   println!("Mod: {} % {} = {} ", value1, value2, modulus);

   let enroll = true;
   println!("Enrollment Status  {} ",enroll);

   let val1:f32 = 55.5;
   let val2:u8 = 34;
   let val3 =  val1 as u8 + val2;
   println!("Value 3 is  {} ",val3);
     let emoji = '\u{1F633}';
    println!("{}",  emoji);

}

