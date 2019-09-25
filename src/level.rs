use rand;
use rand::prelude::*;
use std::fmt;
use room::Room;
pub struct Level {
    width: i32,
    height: i32,
    board: Vec<Vec<i32>>,
    rooms: Vec<Room>
}

impl Level {
    pub fn new(width: i32, height: i32) -> Self {
        let mut board = Vec::new();
        for _ in 0..height {
            let row = vec![0; width as usize];
            board .push(row);
        }
        Level {
            width, height, board, rooms: Vec::new()
        }
    }

    pub fn place_rooms(&mut self, rng: &mut StdRng) {
        
    }
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in 0..self.height as usize {
            for col in 0..self.width as usize {
                write!(f, "{:?}", self.board[row][col])?
            }
            write!(f, "\n")?
        }

        Ok(())
    }
}
