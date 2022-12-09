pub enum Direction {
    Up(u8),
    Down(u8),
    Left(u8),
    Right(u8), 
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub struct Coordinate {
    x: i8,
    y: i8,
}

pub struct Rope {
    head: Coordinate,
    tail: Coordinate,
}

impl Rope {
    pub fn initialize() -> Self {
        Self {
            head: Coordinate { x: 0, y: 0 },
            tail: Coordinate { x: 0, y: 0 },
        }
    }

    pub fn head(self) -> Coordinate {
        self.head
    }

    pub fn tail(self) -> Coordinate {
        self.tail
    }
}

pub fn move_rope_once(rope: Rope, steps: u8) -> Rope {

}

pub fn move_rope(rope: Rope, direction: &Direction) -> () {
    match direction {
        Direction::Up(steps) => {
            
        }
        Direction::Down(steps) => return,
        Direction::Left(steps) => return,
        Direction::Right(steps) => return,
    }
}