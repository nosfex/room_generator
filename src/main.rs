use rand;
// import crates
use sha2;

// arrayref supplies a macro, so add annotation
#[macro_use]
extern crate arrayref;

mod room;
mod level;

// use the sha functions
use sha2::{ Sha256, Digest };
use rand::prelude::*;
use level::Level;

// turn a string into a string 64 characters in length
fn create_hash(text: &str) -> String {
    let mut hasher = Sha256::default();
    hasher.input(text.as_bytes());
    format!("{:x}", hasher.result())
}

fn main() {

    let hash = create_hash("manuelneuerasdasweeperkeeper");
    let seed = array_ref!(hash.as_bytes(), 0, 32);
    let mut rng: StdRng = SeedableRng::from_seed(*seed);

    let mut level = Level::new(48, 40);
    level.place_rooms(&mut rng);
    level.place_corridors(&mut rng);
    println!("{}", level);
}
