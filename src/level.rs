
use rand::prelude::*;
use crate::room::Room;
use std::fmt;
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
            board.push(row);
        }
        Level {
            width, height, board, rooms: Vec::new()
        }
    }

    pub fn place_rooms(&mut self, rng: &mut StdRng) {
        let mut rng = rand::thread_rng();

        let max_rooms = 10;
        let min_room_width = 4;
        let max_room_width = 8;
        let min_room_height = 5;
        let max_room_height = 12;

        for _ in 0..max_rooms {
            let mut x = rng.gen_range(0, self.width);
            let mut y = rng.gen_range(0, self.height);

            let width = rng.gen_range(min_room_width, max_room_width);
            let height = rng.gen_range(min_room_height, max_room_height);

            if x + width > self.width {
                x = self.width - width;
            }

            if y + height > self.height {
                y = self.height - height;
            }

            let mut collides = false;
            let room = Room::new(x, y , width, height);

            for other_room in &self.rooms {
                if room.intersects(&other_room) {
                    collides = true;
                    break;
                }
            }

            if !collides {
                self.add_room(&room);
            }
        }
    }

    fn add_room(&mut self, room: &Room) {
        for row in 0..room.height {
            for col in 0..room.width {
                let y = (room.y + row) as usize;
                let x = (room.x + col) as usize;

                self.board[y][x] = 1;
            }
        }
        self.rooms.push(*room);
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
