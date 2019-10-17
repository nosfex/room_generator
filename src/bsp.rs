use crate::room::Room;
use rand::*;
struct Leaf {
    min_size: i32,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    left_child: Option<Box<Leaf>>,
    right_child: Option<Box<Leaf>>,
    room: Option<Room>
}

impl Leaf {
    pub fn new (x: i32, y: i32, width: i32, height: i32, min_size: i32) -> Self {
        Leaf {
            min_size,
            x,
            y,
            width,
            height,
            left_child: None,
            right_child: None,
            room: None
        }
    }

    fn split(&mut self, rng: &mut StdRng) -> bool {
        let mut split_horz = match rng.gen_range(0,2) {
            0 => false,
            _ => true
        };
        
        true
    }
}
