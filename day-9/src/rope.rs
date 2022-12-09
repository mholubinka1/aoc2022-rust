#[derive(Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right, 
}

pub struct Movement {
    pub direction: Direction,
    pub steps: u8,
}

impl Movement {
    pub fn new(direction: Direction, steps: u8) -> Self {
        Self {
            direction,
            steps,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

impl Coordinate {
    pub fn is_next_to(self, other: Self) -> bool {
        self.x.abs_diff(other.x) <= 1 && self.y.abs_diff(other.y) <= 1
    }

    pub fn move_once(self, direction: &Direction) -> Self {
        match direction {
            Direction::Up => Self { x: self.x, y: self.y + 1 },
            Direction::Down => Self { x: self.x, y: self.y - 1 },
            Direction::Left => Self { x: self.x - 1, y: self.y },
            Direction::Right => Self { x: self.x + 1, y: self.y },
        }
    }

    pub fn move_towards(self, other: Self) -> Self {
        let x = self.x + (other.x - self.x).signum();
        let y = self.y + (other.y - self.y).signum();
        Self {
            x,
            y,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Rope {
    length: usize,
    parts: Vec<Coordinate>,
}

impl Rope {
    pub fn initialize(len: usize) -> Self {
        let mut parts = vec![];
        for _ in 0..len {
            parts.push(Coordinate { x: 0, y: 0 })
        }
        Self {
            length: len,
            parts
        }
    }

    pub fn head(&self) -> Coordinate {
        self.parts[0]
    }

    pub fn tail(&self) -> Coordinate {
        self.parts[self.length-1]
    }
}

pub trait MoveRopeOnce {
    fn move_rope_once(&mut self, direction: &Direction) -> ();
}

impl MoveRopeOnce for Rope {
    fn move_rope_once(&mut self, direction: &Direction) -> () {
        let new_head = self.head().move_once(&direction);
        self.parts[0] = new_head;

        for i in 1..self.length {
            let this_head = self.parts[i - 1];
            let this_tail = self.parts[i];
            let new_tail = {
                match this_tail.is_next_to(this_head) {
                    true => this_tail,
                    false => this_tail.move_towards(this_head),
                }
            };
            self.parts[i] = new_tail;
        }
    }
}