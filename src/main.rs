
use std::io;


#[allow(unused)]
fn main() {
    println!("Hello, world!");
    print!("Press Enter to exit...");
    let mut input = String::new();
    io::stdin().read_line(&mut input);
}
