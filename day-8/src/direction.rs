#[derive(Copy, Clone)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub static DIRECTION: [Direction; 4] = [
    Direction::North,
    Direction::East,
    Direction::South,
    Direction::West,
];

pub fn find_trees(tree_grid: &Vec<Vec<u8>>, r: usize, c: usize, direction: Direction) -> Vec<&u8> {
    match direction {
        Direction::North => tree_grid[0..r].iter().map(|s| &s[c]).rev().collect::<Vec<_>>(),
        Direction::East => tree_grid[r][c + 1..tree_grid[0].len()].iter().collect::<Vec<_>>(),
        Direction::South => tree_grid[r + 1..tree_grid.len()].iter().map(|s| &s[c]).rev().collect::<Vec<_>>(),
        Direction::West => tree_grid[r][0..c].iter().rev().collect::<Vec<_>>(),
    }
}

