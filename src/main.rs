use rand;
// import crates
use rand::distributions::Alphanumeric;
use sha2;
// arrayref supplies a macro, so add annotation
#[macro_use]
extern crate arrayref;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
use clap::{App, Arg};
mod level;
#[macro_use]
mod room;
mod draw;
mod roomscorridors;
mod bsp;
// use the sha functions
use level::Level;
use rand::prelude::*;
use sha2::{Digest, Sha256};
use draw::{ draw };
use roomscorridors::{RoomsCorridors};
use bsp::{BspLevel};
// turn a string into a string 64 characters in length
fn create_hash(text: &str) -> String {
    let mut hasher = Sha256::default();
    hasher.input(text.as_bytes());
    format!("{:x}", hasher.result())
}

enum Algorithm {
    Bsp,
    Rooms
}

fn main() {
    let matches = App::new("Dungeon")
        .version("1.0")
        .author("Gerardito")
        .arg(
            Arg::with_name("text")
                .short("t")
                .long("text")
                .takes_value(true)
                .help("A hash string to use as seed"),
        )
        .arg(
            Arg::with_name("seed")
                .short("s")
                .long("seed")
                .takes_value(true)
                .help("An existing seed. Must be 32 characters"),
        )
        .arg(
            Arg::with_name("algo")
                .short("a")
                .long("algorithm")
                .takes_value(true)
                .possible_values(&["rooms", "bsp"])
                .default_value("rooms")
                .help("The type of proc algo to use"))
        .get_matches();

    let seed: String = match matches.value_of("seed") {
        Some(text) =>
        {
            if text.chars().count() < 32 {
                panic!("Seed must be 32 characters long, use -t option to create new seed")
            }
            text.to_string()
        },
        None => {
            match matches.value_of("text") {
                Some(text) => create_hash(&text),
                None => create_hash(&thread_rng().sample_iter(&Alphanumeric).take(32).collect::<String>())
            }
        }
    };

    let seed_u8 = array_ref!(seed.as_bytes(), 0, 32);
    let mut rng: StdRng = SeedableRng::from_seed(*seed_u8);

    let method = match matches.value_of("algo").expect("Default algorithm not set") {
        "bsp" => Algorithm::Bsp,
        "rooms" => Algorithm::Rooms,
        _ => unreachable![]
    };

    let board_width = 48;
    let board_height = 40;

    let level = match method {
        Algorithm::Rooms => RoomsCorridors::new(board_width, board_height, &seed, &mut rng),
        Algorithm::Bsp => BspLevel::new(board_width, board_height, &seed, &mut rng)
    };

    println!("{}", level);
    let serialised = serde_json::to_string(&level).unwrap();
    println!("{}", level);
    println!("{:?}", serialised);

    draw(&level, "img", "level").unwrap();
}
