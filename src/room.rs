use crate::level::{Tile};

macro_rules! room {
    (
      $([$( $x:expr ),*]),*
    ) => (
      vec![$( vec![$(match $x {
          1 => Tile::Walkable,
          _ => Tile::Empty,
      }),*]),*]
  )
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}
#[derive(Clone, Serialize)]
pub struct Room {
    pub x: i32,
    pub y: i32,
    pub x2: i32,
    pub y2: i32,
    pub width: i32,
    pub height: i32,
    pub centre: Point,

    pub layout: Vec<Vec<Tile>>
}

impl Room {
    pub fn new(x: i32, y: i32, width: i32, height: i32, layout: Option<Vec<Vec<Tile>>>) -> Self {

        let tiles = match layout {
            Some(tiles) => tiles,
            None => {
                let mut board = vec![];
                for _ in 0..height {
                    let row = vec![Tile::Walkable; width as usize];
                    board.push(row);
                }
                board
            }
        };

        Room {
            x,
            y,
            x2: x + width,
            y2: y + height,
            width,
            height,
            centre: Point {
                x: x + (width / 2),
                y: y + (height / 2),
            },
            layout: tiles
        }
    }

    pub fn intersects(&self, other: &Self) -> bool {
        self.x <= other.x2 && self.x2 >= other.x && self.y <= other.y2 && self.y2 >= other.y
    }
}
