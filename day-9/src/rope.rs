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

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

impl Coordinate {
    pub fn is_next_to(self, other: Self) -> bool {
        self.x.abs_diff(other.x) < 2 && self.y.abs_diff(other.y) < 2
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

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
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

    pub fn new(head: Coordinate, tail: Coordinate) -> Self {
        Self {
            head,
            tail,
        }
    }

    pub fn head(self) -> Coordinate {
        self.head
    }

    pub fn tail(self) -> Coordinate {
        self.tail
    }
}

pub fn move_rope_once(rope: Rope, direction: Direction) -> Rope {
    let new_head = rope.head.move_once(&direction);
    let new_tail = {
        match rope.tail.is_next_to(new_head) {
            true => rope.tail.clone(),
            false => rope.tail.move_towards(rope.head),
        }
    };
    Rope::new(new_head, new_tail)
}

/*pub fn move_rope(rope: Rope, movement: &Movement) -> Rope {
    let mut rope = rope.clone();
    for _ in 0..movement.steps {
        rope = move_rope_once(rope, movement.direction);
    }
    rope
}*/