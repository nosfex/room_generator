use crate::room::Room;
use rand;

use serde::{Serialize, Serializer};
use std::fmt;
#[derive(Serialize)]
pub struct Level {
    pub width: i32,
    pub height: i32,
    pub board: Vec<Vec<Tile>>,
    pub tile_size: i32,
    pub rooms: Vec<Room>,
    hash: String,
}

impl Level {
    pub fn new(width: i32, height: i32, hash: &String) -> Self {
        let mut board = Vec::new();
        for _ in 0..height {
            let row = vec![Tile::Empty; width as usize];
            board.push(row);
        }
        Level {
            width,
            height,
            board,
            tile_size: 16,
            rooms: Vec::new(),
            hash: hash.clone(),
        }
    }

    pub fn add_room(&mut self, room: &Room) {
        for row in 0..room.layout.len() {
            for col in 0..room.layout[row].len() {
                let y = room.y as usize + row;
                let x = room.x as usize + col;

                self.board[y][x] = room.layout[row][col];
            }
        }

        self.rooms.push(room.clone());
    }
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in 0..self.height as usize {
            for col in 0..self.width as usize {
                write!(f, "{}", self.board[row][col])?
            }
            write!(f, " \n")?
        }

        Ok(())
    }
}

#[derive(Clone, Copy)]
pub enum Tile {
    Empty,
    Walkable,
}

impl Serialize for Tile {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Tile::Empty => serializer.serialize_i32(0),
            Tile::Walkable => serializer.serialize_i32(1),
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Tile::Empty => write!(f, "0"),
            Tile::Walkable => write!(f, "1"),
        }
    }
}
