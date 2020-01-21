mod pk {
    pub mod sindh {
        pub fn education() {
            println!("We are in pak:sindh:edu ");
        }
    }
}
mod print;  // in print.rs
use pk::sindh::education;  // in main.rs

use pakistan;  // external (on system)

fn main() {
    education();
    print::kpk::peshawar();
    pakistan::islamabad::piaic();
        
}
