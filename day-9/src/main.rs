use std::collections::HashSet;

mod rope;

use rope::{Movement, Direction, Coordinate, Rope, MoveRopeOnce};

const SAMPLE: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

fn load_input() -> String {
    std::fs::read_to_string("input").unwrap()
}

fn parse_movement(input_line: &str) -> Movement {
    match input_line.split_once(' ') {
        Some(("U", number)) => Movement::new(
            Direction::Up,
            number.parse::<u8>().unwrap()
        ),
        Some(("D", number)) => Movement::new(
            Direction::Down,
            number.parse::<u8>().unwrap()
        ),
        Some(("L", number)) => Movement::new(
            Direction::Left,
            number.parse::<u8>().unwrap()
        ),
        Some(("R", number)) => Movement::new(
            Direction::Right,
            number.parse::<u8>().unwrap()
        ),
        _ => unreachable!(),
    }
}

fn create_movements(input: String) -> Vec<Movement> {
    let mut movements = vec![];
    for line in input.lines() {
        movements.push(parse_movement(line));
    }
    movements
}

fn apply_movements(movements: &Vec<Movement>) -> HashSet<Coordinate> {
    let mut visited = HashSet::<Coordinate>::new();
    let mut rope = Rope::initialize(10);
    for movement in movements {
        for _ in 0..movement.steps {
            rope.move_rope_once(&movement.direction);
            visited.insert(rope.tail());
        }
    }
    visited
}

fn main() {
    //let input = SAMPLE.to_string();
    let input = load_input();
    let movements = create_movements(input);
    let visited = apply_movements(&movements);
    println!("{}", visited.len());
}