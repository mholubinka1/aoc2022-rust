use std::collections::HashSet;

mod rope;

use rope::{Direction, Coordinate, Rope, move_rope};

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

fn parse_movement(input_line: &str) -> Direction {
    match input_line.split_once(' ') {
        Some(("U", number)) => Direction::Up(number.parse::<u8>().unwrap()),
        Some(("D", number)) => Direction::Down(number.parse::<u8>().unwrap()),
        Some(("L", number)) => Direction::Left(number.parse::<u8>().unwrap()),
        Some(("R", number)) => Direction::Right(number.parse::<u8>().unwrap()),
        _ => unreachable!(),
    }
}

fn create_movements(input: String) -> Vec<Direction> {
    let mut movements = vec![];
    for line in input.lines() {
        movements.push(parse_movement(line));
    }
    movements
}

fn apply_movements(movements: &Vec<Direction>) -> HashSet<Coordinate> {
    let mut visited = HashSet::<Coordinate>::new();
    let mut rope = Rope::initialize();
    for movement in movements {
        let _ = move_rope(rope, movement);
        visited.insert(rope.tail());
    }
    visited
}

fn main() {
    //let input = SAMPLE.to_string();
    let input = load_input();
    let movements = create_movements(input);
    let visited = apply_movements(movements);
    println!("{}", SAMPLE);
    println!("{}", input);
}