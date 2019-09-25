extern crate rand;
mod level;
use level::Level;
fn main() {
    println!("Hello, world!");
    let level = Level::new(10, 8);
    println!("{}", level);
}
