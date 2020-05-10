mod lunch;
//use lunch::menu;
//use lunch::menu::dinner;
use lunch::menu::{self,dinner};

fn main() {
    println!("We need Food");
    lunch::menu::dinner();
    menu::dinner();
    dinner(); // idiomatic path

}

// Library
// Github Account
// Git install
// Smart Git/ Git command (terminal)
// firstwelcome clone
// Readme
// Cargo.toml