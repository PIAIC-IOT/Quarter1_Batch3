extern crate short_crypt;
use short_crypt::ShortCrypt;

fn main() {
    let sc = ShortCrypt::new("PIAIC");
    println!("PEM tag: {:#?}", sc);
    let data = sc.encrypt_to_url_component("welcome");
    println!("Encrypted data: {}", data);

}


//tinyurl.com/iotjan26
